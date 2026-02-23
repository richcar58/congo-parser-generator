package org.congocc.codegen.rust;

import java.util.*;

import org.congocc.core.*;
import org.congocc.parser.tree.BaseNode;
import org.congocc.parser.tree.ExpansionChoice;
import org.congocc.parser.tree.ExpansionWithParentheses;
import org.congocc.parser.tree.Terminal;
import org.congocc.parser.tree.ZeroOrMore;
import org.congocc.parser.tree.ZeroOrOne;

/**
 * Analyzes a BNFProduction's expansion tree and returns structured metadata
 * for Rust code generation. This enables CTL templates to generate typed
 * operator enums, production-specific node struct fields, and correct
 * parsing logic.
 *
 * Recognized patterns:
 * - ROOT: X : Y &lt;EOF&gt;
 * - INFIX: X : Y ((op1 | op2) Y)*
 * - SEPARATOR: X : Y (sep Y)*
 * - PREFIX: X : &lt;TOKEN&gt; X | Y
 * - CHOICE: X : &lt;A&gt; | &lt;B&gt; | ... | "(" Y ")"
 * - OPTIONAL_SUFFIX: X : Y (complex_suffix)?
 * - GENERIC: Anything else
 */
public class RustProductionInfo {

    public enum PatternType {
        ROOT, INFIX, SEPARATOR, PREFIX, CHOICE, OPTIONAL_SUFFIX, GENERIC
    }

    // ----- Shared fields -----
    private final String productionName;
    private PatternType patternType = PatternType.GENERIC;

    // For ROOT pattern
    private String rootChildName;

    // For INFIX pattern: X : Y ((op1 | op2) Y)*
    private String infixChildName;
    private String operatorEnumName;
    private final List<Map<String, String>> operatorVariants = new ArrayList<>();

    // For SEPARATOR pattern: X : Y (sep Y)*
    private String separatorChildName;
    private String separatorTokenName;

    // For PREFIX pattern: <TOKEN> X | Y
    private String prefixTokenName;
    private String prefixSelfName;
    private String prefixAltChildName;
    private String boolFieldName;

    // For CHOICE pattern: various terminal alternatives
    private final List<Map<String, Object>> choiceAlternatives = new ArrayList<>();
    private Expansion resolvedExpansion; // The expansion after resolveEffective()

    // For OPTIONAL_SUFFIX pattern: X : Y (suffix)?
    private String suffixChildName;
    private String suffixEnumName;
    private final List<Map<String, String>> suffixVariants = new ArrayList<>();
    private final List<Map<String, Object>> suffixAlternatives = new ArrayList<>();

    private RustProductionInfo(String name) {
        this.productionName = name;
    }

    // ===== Public accessors (used from CTL templates via Map-like access) =====

    public String getProductionName() { return productionName; }
    public String getPattern() { return patternType.name().toLowerCase(); }
    public PatternType getPatternType() { return patternType; }

    // Root
    public String getRootChildName() { return rootChildName; }

    // Infix
    public String getInfixChildName() { return infixChildName; }
    public String getOperatorEnumName() { return operatorEnumName; }
    public List<Map<String, String>> getOperatorVariants() { return operatorVariants; }
    public boolean getHasOperatorEnum() { return !operatorVariants.isEmpty(); }

    // Separator
    public String getSeparatorChildName() { return separatorChildName; }
    public String getSeparatorTokenName() { return separatorTokenName; }

    // Prefix
    public String getPrefixTokenName() { return prefixTokenName; }
    public String getPrefixSelfName() { return prefixSelfName; }
    public String getPrefixAltChildName() { return prefixAltChildName; }
    public String getBoolFieldName() { return boolFieldName; }
    public boolean getHasBoolField() { return boolFieldName != null; }

    // Choice
    public List<Map<String, Object>> getChoiceAlternatives() { return choiceAlternatives; }
    public Expansion getResolvedExpansion() { return resolvedExpansion; }
    /** Returns the token names of all terminal alternatives (for grouped emit). */
    public List<String> getTerminalTokenNames() {
        List<String> names = new ArrayList<>();
        for (Map<String, Object> alt : choiceAlternatives) {
            if ("terminal".equals(alt.get("type"))) {
                names.add((String) alt.get("tokenName"));
            }
        }
        return names;
    }

    // Optional suffix
    public String getSuffixChildName() { return suffixChildName; }
    public String getSuffixEnumName() { return suffixEnumName; }
    public List<Map<String, String>> getSuffixVariants() { return suffixVariants; }
    public List<Map<String, Object>> getSuffixAlternatives() { return suffixAlternatives; }
    public boolean getHasSuffixEnum() { return suffixEnumName != null && !suffixVariants.isEmpty(); }

    // ===== Analysis entry point =====

    public static RustProductionInfo analyze(BNFProduction production) {
        RustProductionInfo info = new RustProductionInfo(production.getName());
        Expansion exp = production.getExpansion();
        if (exp == null) {
            return info;
        }

        // Pre-process: strip CodeBlocks and unwrap wrappers
        exp = resolveEffective(exp);
        info.resolvedExpansion = exp;

        String cls = exp.getSimpleName();

        if ("ExpansionSequence".equals(cls)) {
            analyzeSequence(info, production, (ExpansionSequence) exp);
        } else if ("ExpansionChoice".equals(cls)) {
            analyzeTopLevelChoice(info, production, (ExpansionChoice) exp);
        } else {
            info.patternType = PatternType.GENERIC;
        }

        return info;
    }

    // ===== Pattern detection for sequences =====

    private static void analyzeSequence(RustProductionInfo info, BNFProduction production, ExpansionSequence seq) {
        List<Expansion> units = filterSyntactic(seq.getUnits());
        if (units.size() == 2) {
            Expansion first = units.get(0);
            Expansion second = units.get(1);

            // ROOT: NonTerminal <EOF>
            if (isNonTerminal(first) && isTerminalWithLabel(second, "EOF")) {
                info.patternType = PatternType.ROOT;
                info.rootChildName = getNonTerminalName(first);
                return;
            }

            // INFIX or SEPARATOR: NonTerminal (... NonTerminal)*
            if (isNonTerminal(first) && isZeroOrMore(second)) {
                Expansion inner = getNestedExpansion(second);
                if (inner != null) {
                    // Resolve inner to handle SCAN wrappers, parens, CodeBlocks
                    inner = resolveEffective(inner);

                    if ("ExpansionSequence".equals(inner.getSimpleName())) {
                        List<Expansion> innerUnits = filterSyntactic(((ExpansionSequence) inner).getUnits());
                        if (innerUnits.size() >= 2) {
                            Expansion opPart = unwrapParens(innerUnits.get(0));
                            Expansion childPart = innerUnits.get(innerUnits.size() - 1);

                            if (isNonTerminal(childPart)) {
                                // Check if opPart is a choice of terminals (INFIX)
                                if ("ExpansionChoice".equals(opPart.getSimpleName())) {
                                    List<String[]> terminalLabels = extractTerminalLabelsFromChoice((ExpansionChoice) opPart);
                                    if (terminalLabels != null && terminalLabels.size() >= 2) {
                                        info.patternType = PatternType.INFIX;
                                        info.infixChildName = getNonTerminalName(first);
                                        info.operatorEnumName = generateEnumName(production.getName());
                                        for (String[] labelInfo : terminalLabels) {
                                            Map<String, String> variant = new LinkedHashMap<>();
                                            variant.put("variantName", tokenNameToVariant(labelInfo[0]));
                                            variant.put("tokenName", rustifyTokenName(labelInfo[0]));
                                            variant.put("display", labelInfo[1]);
                                            info.operatorVariants.add(variant);
                                        }
                                        return;
                                    }
                                }

                                // Check if opPart is a single terminal (SEPARATOR)
                                if (isTerminal(opPart)) {
                                    info.patternType = PatternType.SEPARATOR;
                                    info.separatorChildName = getNonTerminalName(first);
                                    info.separatorTokenName = rustifyTokenName(getTerminalLabel(opPart));
                                    return;
                                }
                            }
                        }
                    }

                    // INFIX from flattened choice: each alt is [terminal, child]
                    if ("ExpansionChoice".equals(inner.getSimpleName())) {
                        ExpansionChoice choice = (ExpansionChoice) inner;
                        List<ExpansionSequence> alts = choice.getChoices();
                        String childName = null;
                        List<String[]> terminalLabels = new ArrayList<>();
                        boolean isInfix = true;

                        for (ExpansionSequence alt : alts) {
                            List<Expansion> altUnits = filterSyntactic(alt.getUnits());
                            if (altUnits.size() == 2 && isTerminal(altUnits.get(0)) && isNonTerminal(altUnits.get(1))) {
                                Terminal t = (Terminal) altUnits.get(0);
                                String ntName = getNonTerminalName(altUnits.get(1));
                                if (childName == null) childName = ntName;
                                if (!childName.equals(ntName)) { isInfix = false; break; }
                                String display = t.getRegexp().getLiteralString();
                                if (display == null) display = tokenNameToDisplay(t.getLabel());
                                terminalLabels.add(new String[]{t.getLabel(), display});
                            } else {
                                isInfix = false;
                                break;
                            }
                        }

                        if (isInfix && terminalLabels.size() >= 2) {
                            info.patternType = PatternType.INFIX;
                            info.infixChildName = getNonTerminalName(first);
                            info.operatorEnumName = generateEnumName(production.getName());
                            for (String[] labelInfo : terminalLabels) {
                                Map<String, String> variant = new LinkedHashMap<>();
                                variant.put("variantName", tokenNameToVariant(labelInfo[0]));
                                variant.put("tokenName", rustifyTokenName(labelInfo[0]));
                                variant.put("display", labelInfo[1]);
                                info.operatorVariants.add(variant);
                            }
                            return;
                        }
                    }
                }
            }

            // OPTIONAL_SUFFIX: NonTerminal (suffix)?
            if (isNonTerminal(first) && isZeroOrOne(second)) {
                info.patternType = PatternType.OPTIONAL_SUFFIX;
                info.suffixChildName = getNonTerminalName(first);
                analyzeOptionalSuffix(info, production, second);
                return;
            }
        }

        info.patternType = PatternType.GENERIC;
    }

    // ===== Pattern detection for top-level choices =====

    private static void analyzeTopLevelChoice(RustProductionInfo info, BNFProduction production, ExpansionChoice choice) {
        List<ExpansionSequence> choices = choice.getChoices();

        // PREFIX: <TOKEN> Self | Other
        if (choices.size() == 2) {
            ExpansionSequence firstAlt = choices.get(0);
            ExpansionSequence secondAlt = choices.get(1);
            List<Expansion> firstUnits = filterSyntactic(firstAlt.getUnits());
            List<Expansion> secondUnits = filterSyntactic(secondAlt.getUnits());

            if (firstUnits.size() == 2 && isTerminal(firstUnits.get(0)) && isNonTerminal(firstUnits.get(1))) {
                String ntName = getNonTerminalName(firstUnits.get(1));
                if (ntName.equals(production.getName())) {
                    // This IS a prefix pattern: <TOKEN> Self | Other
                    info.patternType = PatternType.PREFIX;
                    info.prefixTokenName = rustifyTokenName(getTerminalLabel(firstUnits.get(0)));
                    info.prefixSelfName = ntName;

                    // Determine the boolean field name based on the prefix token
                    String tokenName = info.prefixTokenName;
                    if ("NOT".equals(tokenName) || "MINUS".equals(tokenName)) {
                        info.boolFieldName = "is_negated";
                    } else {
                        info.boolFieldName = "has_" + tokenName.toLowerCase();
                    }

                    // Find the alternate child (the non-self reference)
                    if (secondUnits.size() == 1 && isNonTerminal(secondUnits.get(0))) {
                        info.prefixAltChildName = getNonTerminalName(secondUnits.get(0));
                    }
                    return;
                }
            }
        }

        // CHOICE: multiple alternatives (terminals, nonterminals, sequences starting with terminals)
        info.patternType = PatternType.CHOICE;
        for (ExpansionSequence alt : choices) {
            Map<String, Object> altInfo = new LinkedHashMap<>();
            List<Expansion> altUnits = filterSyntactic(alt.getUnits());

            // Resolve single-element wrappers (e.g., parens around nonterminal + code block)
            if (altUnits.size() == 1) {
                Expansion resolved = resolveEffective(altUnits.get(0));
                if ("ExpansionSequence".equals(resolved.getSimpleName())) {
                    altUnits = filterSyntactic(((ExpansionSequence) resolved).getUnits());
                } else {
                    altUnits = new ArrayList<>();
                    altUnits.add(resolved);
                }
            }

            if (altUnits.size() == 1 && isTerminal(altUnits.get(0))) {
                // Single terminal alternative
                altInfo.put("type", "terminal");
                altInfo.put("tokenName", rustifyTokenName(getTerminalLabel(altUnits.get(0))));
            } else if (altUnits.size() == 1 && isNonTerminal(altUnits.get(0))) {
                // Single nonterminal alternative
                altInfo.put("type", "nonterminal");
                altInfo.put("productionName", getNonTerminalName(altUnits.get(0)));
            } else if (altUnits.size() >= 3 && isTerminal(altUnits.get(0)) && isTerminal(altUnits.get(altUnits.size() - 1))) {
                // Parenthesized expression: "(" NonTerminal ")"
                String firstLiteral = getTerminalLiteral(altUnits.get(0));
                String lastLiteral = getTerminalLiteral(altUnits.get(altUnits.size() - 1));
                if ("(".equals(firstLiteral) && ")".equals(lastLiteral) && altUnits.size() == 3) {
                    Expansion middle = altUnits.get(1);
                    if (isNonTerminal(middle)) {
                        altInfo.put("type", "parenthesized");
                        altInfo.put("innerProdName", getNonTerminalName(middle));
                        altInfo.put("lparenToken", rustifyTokenName(getTerminalLabel(altUnits.get(0))));
                        altInfo.put("rparenToken", rustifyTokenName(getTerminalLabel(altUnits.get(2))));
                    } else {
                        altInfo.put("type", "sequence");
                    }
                } else {
                    altInfo.put("type", "sequence");
                }
            } else {
                altInfo.put("type", "sequence");
            }
            info.choiceAlternatives.add(altInfo);
        }

        // If any alternative is unhandled ("sequence"), fall back to GENERIC
        for (Map<String, Object> altInfo : info.choiceAlternatives) {
            if ("sequence".equals(altInfo.get("type"))) {
                info.patternType = PatternType.GENERIC;
                info.choiceAlternatives.clear();
                return;
            }
        }
    }

    // ===== Optional suffix analysis =====

    private static void analyzeOptionalSuffix(RustProductionInfo info, BNFProduction production, Expansion zeroOrOne) {
        Expansion nested = getNestedExpansion(zeroOrOne);
        if (nested == null) return;

        nested = unwrapParens(nested);

        if (!"ExpansionChoice".equals(nested.getSimpleName())) {
            // Not a choice - just a simple optional
            return;
        }

        ExpansionChoice choice = (ExpansionChoice) nested;
        List<ExpansionSequence> alts = choice.getChoices();

        // Determine if this looks like a comparison-like pattern with a typed enum
        // by checking if each alternative starts with a distinct token or group of tokens
        boolean canGenerateEnum = true;
        List<Map<String, String>> allVariants = new ArrayList<>();
        List<Map<String, Object>> allAlts = new ArrayList<>();

        for (ExpansionSequence alt : alts) {
            List<Expansion> units = filterSyntactic(alt.getUnits());
            if (units.isEmpty()) {
                canGenerateEnum = false;
                break;
            }

            Expansion first = unwrapParens(units.get(0));
            Map<String, Object> altInfo = new LinkedHashMap<>();

            if ("ExpansionChoice".equals(first.getSimpleName())) {
                // Group of operator tokens: (<EQ> | <NE> | ...) followed by nonterminal
                List<String[]> termLabels = extractTerminalLabelsFromChoice((ExpansionChoice) first);
                if (termLabels != null) {
                    altInfo.put("type", "comparison_group");
                    List<String> tokenNames = new ArrayList<>();
                    for (String[] tl : termLabels) {
                        Map<String, String> v = new LinkedHashMap<>();
                        v.put("variantName", tokenNameToVariant(tl[0]));
                        v.put("tokenName", rustifyTokenName(tl[0]));
                        v.put("display", tl[1]);
                        allVariants.add(v);
                        tokenNames.add(rustifyTokenName(tl[0]));
                    }
                    altInfo.put("tokenNames", tokenNames);

                    // Check for right-hand child
                    if (units.size() >= 2 && isNonTerminal(units.get(units.size() - 1))) {
                        altInfo.put("rightChildName", getNonTerminalName(units.get(units.size() - 1)));
                    }
                } else {
                    canGenerateEnum = false;
                    break;
                }
            } else if (isTerminal(first)) {
                String tokenName = rustifyTokenName(getTerminalLabel(first));
                altInfo.put("type", "keyword");
                altInfo.put("leadingToken", tokenName);

                // Check for IS [NOT] NULL pattern
                if ("IS".equals(tokenName)) {
                    altInfo.put("type", "is_null");
                    Map<String, String> v1 = new LinkedHashMap<>();
                    v1.put("variantName", "IsNull");
                    v1.put("tokenName", "IS");
                    v1.put("display", "IS NULL");
                    allVariants.add(v1);
                    Map<String, String> v2 = new LinkedHashMap<>();
                    v2.put("variantName", "IsNotNull");
                    v2.put("tokenName", "IS");
                    v2.put("display", "IS NOT NULL");
                    allVariants.add(v2);
                } else {
                    Map<String, String> v = new LinkedHashMap<>();
                    v.put("variantName", tokenNameToVariant(tokenName));
                    v.put("tokenName", rustifyTokenName(tokenName));
                    v.put("display", tokenName);
                    allVariants.add(v);
                }

                // Analyze the rest of the sequence for code generation hints
                if (units.size() >= 2) {
                    List<Map<String, String>> restInfo = new ArrayList<>();
                    for (int i = 1; i < units.size(); i++) {
                        Expansion unit = units.get(i);
                        Map<String, String> unitInfo = new LinkedHashMap<>();
                        if (isTerminal(unit)) {
                            unitInfo.put("type", "terminal");
                            unitInfo.put("tokenName", rustifyTokenName(getTerminalLabel(unit)));
                        } else if (isNonTerminal(unit)) {
                            unitInfo.put("type", "nonterminal");
                            unitInfo.put("name", getNonTerminalName(unit));
                        } else if (isZeroOrOne(unit)) {
                            unitInfo.put("type", "optional");
                            Expansion optNested = getNestedExpansion(unit);
                            if (optNested != null && isTerminal(unwrapParens(optNested))) {
                                unitInfo.put("optionalTokenName", rustifyTokenName(getTerminalLabel(unwrapParens(optNested))));
                            }
                        } else {
                            unitInfo.put("type", "other");
                        }
                        restInfo.add(unitInfo);
                    }
                    altInfo.put("rest", restInfo);
                }
            } else {
                canGenerateEnum = false;
                break;
            }

            allAlts.add(altInfo);
        }

        if (canGenerateEnum && !allVariants.isEmpty()) {
            info.suffixEnumName = generateEnumName(production.getName());
            info.suffixVariants.addAll(allVariants);
            info.suffixAlternatives.addAll(allAlts);
        }
    }

    // ===== Filtering helpers =====

    /**
     * Filter a list of expansions to only those that consume tokens.
     * Removes CodeBlock, RawCode, Assertion, Failure, UncacheTokens, etc.
     */
    private static List<Expansion> filterSyntactic(List<Expansion> units) {
        List<Expansion> result = new ArrayList<>();
        for (Expansion unit : units) {
            if (unit.getMaximumSize() > 0) {
                result.add(unit);
            }
        }
        return result;
    }

    /**
     * Resolve the effective expansion by filtering non-syntactic elements
     * and unwrapping single-element wrappers.
     */
    private static Expansion resolveEffective(Expansion exp) {
        exp = unwrapParens(exp);
        if ("ExpansionSequence".equals(exp.getSimpleName())) {
            List<Expansion> syntactic = filterSyntactic(((ExpansionSequence) exp).getUnits());
            if (syntactic.size() == 1) {
                return resolveEffective(syntactic.get(0));
            }
        }
        return exp;
    }

    // ===== Helper methods =====

    private static boolean isNonTerminal(Expansion exp) {
        return exp instanceof NonTerminal;
    }

    private static boolean isTerminal(Expansion exp) {
        return "Terminal".equals(exp.getSimpleName());
    }

    private static boolean isTerminalWithLabel(Expansion exp, String label) {
        if (!isTerminal(exp)) return false;
        return label.equals(getTerminalLabel(exp));
    }

    private static boolean isZeroOrMore(Expansion exp) {
        return "ZeroOrMore".equals(exp.getSimpleName());
    }

    private static boolean isZeroOrOne(Expansion exp) {
        return "ZeroOrOne".equals(exp.getSimpleName());
    }

    private static String getNonTerminalName(Expansion exp) {
        return ((NonTerminal) exp).getName();
    }

    private static String getTerminalLabel(Expansion exp) {
        Terminal t = (Terminal) exp;
        return t.getLabel();
    }

    private static String getTerminalLiteral(Expansion exp) {
        Terminal t = (Terminal) exp;
        String literal = t.getRegexp().getLiteralString();
        return literal != null ? literal : t.getLabel();
    }

    private static Expansion getNestedExpansion(Expansion exp) {
        if (exp instanceof ExpansionWithNested ewn) {
            return ewn.getNestedExpansion();
        }
        return null;
    }

    private static Expansion unwrapParens(Expansion exp) {
        while ("ExpansionWithParentheses".equals(exp.getSimpleName())) {
            Expansion inner = getNestedExpansion(exp);
            if (inner == null) break;
            exp = inner;
        }
        // Also unwrap single-element sequences
        if ("ExpansionSequence".equals(exp.getSimpleName())) {
            List<Expansion> units = ((ExpansionSequence) exp).getUnits();
            if (units.size() == 1) {
                return unwrapParens(units.get(0));
            }
        }
        return exp;
    }

    /**
     * Extract terminal labels from an ExpansionChoice where every alternative
     * is a single terminal. Returns null if any alternative is not a single terminal.
     * Returns array of [tokenName, displayString] pairs.
     */
    private static List<String[]> extractTerminalLabelsFromChoice(ExpansionChoice choice) {
        List<String[]> result = new ArrayList<>();
        for (ExpansionSequence alt : choice.getChoices()) {
            List<Expansion> units = alt.getUnits();
            if (units.size() != 1 || !isTerminal(units.get(0))) {
                return null;
            }
            Terminal t = (Terminal) units.get(0);
            String label = t.getLabel();
            String display = t.getRegexp().getLiteralString();
            if (display == null) display = tokenNameToDisplay(label);
            result.add(new String[]{label, display});
        }
        return result;
    }

    /**
     * Convert a token name to its display string (the actual character(s) it represents).
     */
    /**
     * Convert _TOKEN_N to TokenN for Rust TokenType enum compatibility.
     */
    private static String rustifyTokenName(String name) {
        if (name.startsWith("_TOKEN_")) {
            return name.replace("_TOKEN_", "Token");
        }
        return name;
    }

    private static String tokenNameToDisplay(String tokenName) {
        return switch (tokenName) {
            case "PLUS" -> "+";
            case "MINUS" -> "-";
            case "STAR" -> "*";
            case "SLASH" -> "/";
            case "EQ" -> "=";
            case "NE" -> "!=";
            case "LT" -> "<";
            case "GT" -> ">";
            case "LE" -> "<=";
            case "GE" -> ">=";
            case "LPAREN" -> "(";
            case "RPAREN" -> ")";
            case "COMMA" -> ",";
            default -> tokenName;
        };
    }

    /**
     * Generate an enum name from a production name.
     * e.g., "AdditiveExpression" -> "AdditiveOp"
     *       "MultiplicativeExpression" -> "MultiplicativeOp"
     */
    private static String generateEnumName(String productionName) {
        String result;
        if (productionName.endsWith("Expression")) {
            result = productionName.substring(0, productionName.length() - "Expression".length()) + "Op";
        } else {
            result = productionName + "Op";
        }
        // Ensure PascalCase for Rust enum naming convention
        return Character.toUpperCase(result.charAt(0)) + result.substring(1);
    }

    /**
     * Convert a token name like "PLUS" to an enum variant name like "Add".
     * Maps common operator tokens to readable names.
     */
    private static String tokenNameToVariant(String tokenName) {
        return switch (tokenName) {
            case "PLUS" -> "Add";
            case "MINUS" -> "Sub";
            case "STAR" -> "Mul";
            case "SLASH" -> "Div";
            case "EQ" -> "Eq";
            case "NE" -> "Ne";
            case "LT" -> "Lt";
            case "GT" -> "Gt";
            case "LE" -> "Le";
            case "GE" -> "Ge";
            case "LIKE" -> "Like";
            case "IN" -> "In";
            case "BETWEEN" -> "Between";
            case "AND" -> "And";
            case "OR" -> "Or";
            case "NOT" -> "Not";
            case "COMMA" -> "Comma";
            default -> {
                // Convert UPPER_CASE to PascalCase
                StringBuilder sb = new StringBuilder();
                boolean capitalize = true;
                for (char c : tokenName.toCharArray()) {
                    if (c == '_') {
                        capitalize = true;
                    } else {
                        sb.append(capitalize ? Character.toUpperCase(c) : Character.toLowerCase(c));
                        capitalize = false;
                    }
                }
                yield sb.toString();
            }
        };
    }
}

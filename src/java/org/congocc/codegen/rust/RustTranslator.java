package org.congocc.codegen.rust;

import java.util.*;

import org.congocc.core.Grammar;
import org.congocc.codegen.Translator;
import org.congocc.parser.Node;
import org.congocc.parser.tree.*;

/**
 * Translator for Rust code generation.
 * Translates Java-like grammar code to idiomatic Rust with arena allocation.
 */
public class RustTranslator extends Translator {

    public RustTranslator(Grammar grammar) {
        super(grammar);
        this.isTyped = true;  // Rust is strongly typed
        this.methodIndent = 4;
        this.fieldIndent = 4;
        this.includeInitializers = false;  // Will handle initialization differently with arena
    }

    /**
     * Translate operators to Rust equivalents.
     * Most operators are the same in Rust and Java.
     */
    public String translateOperator(String operator) {
        // Rust uses same operators as Java for most cases
        return operator;
    }

    /**
     * Translate identifiers from Java conventions to Rust conventions.
     * - null → None (for Option types)
     * - true/false stay the same
     * - this → self
     * - camelCase → snake_case for variables and functions
     */
    @Override
    public String translateIdentifier(String ident, TranslationContext kind) {
        String result = ident;

        if (ident.equals("null")) {
            result = "None";
        }
        else if (ident.equals("this")) {
            result = "self";
        }
        else if (ident.equals("toString")) {
            result = "to_string";
        }
        else if (ident.equals("currentLookaheadToken") ||
                 ident.equals("lastConsumedToken")) {
            result = String.format("self.%s", camelToSnake(ident));
        }
        else if (ident.equals("LEXER_CLASS") || ident.equals(appSettings.getLexerClassName())) {
            result = "Lexer";
        }
        else if (ident.equals("PARSER_CLASS") || ident.equals(appSettings.getParserClassName())) {
            result = "Parser";
        }
        else if (ident.equals("BASE_TOKEN_CLASS") || ident.equals(appSettings.getBaseTokenClassName())) {
            result = "Token";
        }
        else if (ident.equals("THIS_PRODUCTION") || ident.equals("THIS")) {
            result = "this_production";
        }
        else if (ident.startsWith(appSettings.getNodePackage().concat("."))) {
            int prefixLength = appSettings.getNodePackage().length() + 1;
            result = ident.substring(prefixLength);
        }
        else if (ident.startsWith("NODE_PACKAGE.")) {
            result = ident.substring(13);
        }
        // Convert camelCase/PascalCase to snake_case for variables and methods
        else if (kind == TranslationContext.VARIABLE ||
                 kind == TranslationContext.METHOD ||
                 kind == TranslationContext.PARAMETER) {
            // Always convert to snake_case for Rust naming conventions
            // camelToSnake preserves first char case, so we need to lowercase it
            result = camelToSnake(ident).toLowerCase();
        }

        return result;
    }

    /**
     * Translate Java types to Rust types.
     * This handles basic type mappings and prepares for arena-based architecture.
     */
    @Override
    public String translateTypeName(String typeName) {
        String result = typeName;

        switch (typeName) {
            case "boolean":
                result = "bool";
                break;
            case "Boolean":
                result = "Option<bool>";
                break;
            case "int":
            case "Integer":
                result = "i32";
                break;
            case "long":
            case "Long":
                result = "i64";
                break;
            case "float":
            case "Float":
                result = "f32";
                break;
            case "double":
            case "Double":
                result = "f64";
                break;
            case "String":
                result = "String";
                break;
            case "char":
                result = "char";
                break;
            case "void":
                result = "()";
                break;
            // Collections - will be arena-based
            case "List":
            case "ArrayList":
            case "java.util.List":
            case "java.util.ArrayList":
                result = "Vec";  // Generic param will be NodeId for nodes
                break;
            case "Set":
            case "HashSet":
            case "java.util.Set":
            case "java.util.HashSet":
                result = "HashSet";
                break;
            case "Map":
            case "HashMap":
            case "java.util.Map":
            case "java.util.HashMap":
                result = "HashMap";
                break;
            // Node and Token will become IDs
            case "Node":
                result = "NodeId";
                break;
            case "Token":
                result = "TokenId";
                break;
        }

        return result;
    }

    // Note: camelToSnake is inherited as a static method from Translator

    /**
     * Translate Java method invocations to Rust equivalents.
     * This is a placeholder for arena-based method translation.
     */
    @Override
    protected void translateInvocation(ASTInvocation expr, StringBuilder result) {
        // TODO: Implement arena-based method call translation
        // For now, delegate to superclass
        super.translateInvocation(expr, result);
    }

    /**
     * Translate type expressions including generics.
     */
    @Override
    protected void translateType(ASTTypeExpression typeExpr, StringBuilder result) {
        String baseName = translateTypeName(typeExpr.getName());
        result.append(baseName);

        // Handle generic types
        if (typeExpr.getTypeParameters() != null && !typeExpr.getTypeParameters().isEmpty()) {
            result.append('<');
            boolean first = true;
            for (ASTTypeExpression typeArg : typeExpr.getTypeParameters()) {
                if (!first) result.append(", ");
                first = false;
                translateType(typeArg, result);
            }
            result.append('>');
        }
    }
}

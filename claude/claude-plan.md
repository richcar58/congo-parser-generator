● Perfect! I've created a comprehensive implementation plan for adding idiomatic Rust parser generation to CongoCC. The plan details:

  Key Highlights

  Architecture: Arena-based allocation with type-safe indices (NodeId/TokenId) and enum-based AST nodes for true Rust idioms

  Cross-Language Compatibility: RustTranslator will translate Java semantics from existing grammars to idiomatic Rust patterns automatically

  Support Crate: congocc-runtime provides Arena, error types, and common utilities

  Timeline: 12-13 weeks for full implementation with 7 phases

  Generated Code: Zero-cost abstractions, no unsafe code, Result-based error handling, compile-time safety

  The plan includes:
  - Detailed implementation for all components (RustTranslator, templates, runtime crate)
  - Specific file paths and code examples
  - Translation patterns for all Rust challenges (ownership, no inheritance, errors, etc.)
  - Testing strategy with example grammars
  - Phase-by-phase implementation roadmap

─────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
 Ready to code?

 Here is Claude's plan:
╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
 Implementation Plan: Idiomatic Rust Parser Generation for CongoCC

 Overview

 Add Rust code generation support to CongoCC using arena allocation with indices for idiomatic, zero-cost abstractions while maintaining cross-language compatibility with existing Java/Python/C# grammars.

 Approach: Arena-based architecture with enum nodes and Result-based error handling
 Compatibility: Existing grammars work without modification
 Dependencies: Create congocc-runtime support crate for shared utilities

 Architecture Summary

 Generated Code Structure

 // Arena owns all nodes and tokens
 pub struct Arena {
     nodes: Vec<AstNode>,
     tokens: Vec<Token>,
 }

 // Type-safe indices replace pointers
 #[derive(Copy, Clone)] pub struct NodeId(usize);
 #[derive(Copy, Clone)] pub struct TokenId(usize);

 // All node types in one enum (replaces inheritance)
 pub enum AstNode {
     Expression(ExpressionNode),
     Statement(StatementNode),
     // ... all node types
 }

 // Parser takes mutable arena reference
 pub struct Parser<'a> {
     arena: &'a mut Arena,
     lexer: Lexer<'a>,
     current_token: TokenId,
 }

 Error Handling

 // Replace exceptions with Result types
 pub type ParseResult<T> = Result<T, ParseError>;

 impl Parser {
     pub fn parse_expression(&mut self) -> ParseResult<NodeId> {
         // All errors explicitly propagated with ?
     }
 }

 Implementation Components

 1. CongoCC Runtime Support Crate

 Location: New crate congocc-runtime/ (published separately)

 Purpose: Shared utilities for all generated Rust parsers

 Contents:
 - Arena - Node/token allocation with indices
 - NodeId, TokenId - Type-safe index wrappers
 - ParseError - Standard error type
 - TokenType - Common token type trait
 - Collection wrappers (if needed for API compatibility)
 - Utility macros for common patterns

 File: congocc-runtime/src/lib.rs
 pub struct Arena {
     nodes: Vec<AstNode>,
     tokens: Vec<Token>,
 }

 impl Arena {
     pub fn new() -> Self { ... }
     pub fn alloc_node(&mut self, node: AstNode) -> NodeId { ... }
     pub fn alloc_token(&mut self, token: Token) -> TokenId { ... }
     pub fn get_node(&self, id: NodeId) -> &AstNode { ... }
     pub fn get_node_mut(&mut self, id: NodeId) -> &mut AstNode { ... }
 }

 2. CongoCC Java Code Changes

 2.1 Add RUST to CodeLang Enum

 File: /home/rich/git/congo-rustgen/src/grammars/CongoCC.ccc (or wherever CodeLang is defined in grammar)

 Change: Add RUST to the CodeLang enum
 public enum CodeLang {
     JAVA, PYTHON, CSHARP, RUST
 }

 2.2 Create RustTranslator

 File: /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/rust/RustTranslator.java

 Extends: Translator base class

 Key Methods:
 public class RustTranslator extends Translator {
     public RustTranslator(Grammar grammar) {
         super(grammar);
         this.isTyped = true;  // Rust is strongly typed
         this.methodIndent = 4;
         this.fieldIndent = 4;
     }

     @Override
     public String translateIdentifier(String ident, TranslationContext kind) {
         // null → None, this → self, camelCase → snake_case
     }

     @Override
     protected void translateInvocation(ASTInvocation expr, StringBuilder result) {
         // Method calls: receiver.method() → receiver.method()
         // Special cases: list.get(i) → arena.get_node(list).children[i]
     }

     @Override
     protected void translateType(ASTTypeExpression type, StringBuilder result) {
         // Java types → Rust types with arena-based structure
         // List<T> → Vec<NodeId>
         // Token → TokenId
         // null types → Option<T>
     }

     @Override
     public String translateOperator(String op) {
         // || → ||, && → &&, ! → !  (same as Java)
     }

     // Arena-specific translation
     protected void translateNodeAccess(ASTExpression receiver, StringBuilder result) {
         // Wrap node accesses with arena.get_node(id)
     }

     protected void translateNodeAllocation(ASTInvocation newExpr, StringBuilder result) {
         // new Expression() → arena.alloc_node(AstNode::Expression(...))
     }
 }

 Key Translation Patterns:
 - Null checks: if (x == null) → if x.is_none()
 - Object allocation: new Token() → arena.alloc_token(Token::new())
 - Method calls: node.getParent() → arena.get_node(node_id).parent
 - Collections: List<Node> → Vec<NodeId>
 - Inheritance: instanceof Type → pattern matching with match

 2.3 Create RustFormatter (Optional)

 File: /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/rust/RustFormatter.java

 Alternative: Use external rustfmt instead of implementing formatter in Java

 If implemented: Follow pattern of PyFormatter / CSharpFormatter

 2.4 Update FilesGenerator

 File: /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/FilesGenerator.java

 Changes:

 1. Add RUST case to generateAll() method:
 case RUST -> {
     String[] paths = new String[]{
         "lib.rs",
         "arena.rs",
         "tokens.rs",
         "lexer.rs",
         "parser.rs",
         "error.rs",
         "Cargo.toml",
         "test/parse_files.rs"
     };
     for (String p : paths) {
         generate(parserOutputDirectory, p);
     }
 }

 2. Add outputRustFile() method:
 void outputRustFile(String code, Path outputFile) throws IOException {
     // Option 1: Just write and rely on external rustfmt
     Files.writeString(outputFile, code);

     // Option 2: Parse Rust AST and format (complex)
     // Would need RustParser and RustFormatter
 }

 3. Update getTranslatorFor():
 public static Translator getTranslatorFor(Grammar grammar) {
     CodeLang codeLang = grammar.getAppSettings().getCodeLang();
     if (codeLang == PYTHON) return new PythonTranslator(grammar);
     else if (codeLang == CSHARP) return new CSharpTranslator(grammar);
     else if (codeLang == RUST) return new RustTranslator(grammar);
     return new Translator(grammar);
 }

 2.5 Update Translator Base Class

 File: /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/Translator.java

 Add method: getTranslatorFor() factory method (if not present)

 3. Rust Code Generation Templates

 Location: /home/rich/git/congo-rustgen/src/templates/rust/

 3.1 lib.rs.ctl (Module Root)

 //! Generated by CongoCC ${generated_by}
 //! Grammar: ${settings.filename}

 mod arena;
 mod tokens;
 mod lexer;
 mod parser;
 mod error;

 pub use arena::{Arena, NodeId, TokenId};
 pub use tokens::{Token, TokenType};
 pub use lexer::Lexer;
 pub use parser::Parser;
 pub use error::{ParseError, ParseResult};

 // Export AST node types
 pub use parser::{AstNode, ExpressionNode, StatementNode, ...};

 3.2 arena.rs.ctl (Arena Allocator)

 use crate::tokens::Token;

 #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
 pub struct NodeId(pub(crate) usize);

 #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
 pub struct TokenId(pub(crate) usize);

 pub struct Arena {
     pub(crate) nodes: Vec<AstNode>,
     pub(crate) tokens: Vec<Token>,
 }

 impl Arena {
     pub fn new() -> Self {
         Arena {
             nodes: Vec::new(),
             tokens: Vec::new(),
         }
     }

     pub fn alloc_node(&mut self, node: AstNode) -> NodeId {
         let id = NodeId(self.nodes.len());
         self.nodes.push(node);
         id
     }

     pub fn get_node(&self, id: NodeId) -> &AstNode {
         &self.nodes[id.0]
     }

     pub fn get_node_mut(&mut self, id: NodeId) -> &mut AstNode {
         &mut self.nodes[id.0]
     }

     // Similar for tokens...
 }

 3.3 tokens.rs.ctl (Token Types)

 #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
 pub enum TokenType {
 #list lexerData.regularExpressions as regexp
     ${regexp.label},
 #endlist
 }

 #[derive(Debug, Clone)]
 pub struct Token {
     pub token_type: TokenType,
     pub image: String,
     pub begin_offset: usize,
     pub end_offset: usize,
     pub next: Option<TokenId>,
     pub previous: Option<TokenId>,
 }

 impl Token {
     pub fn new(token_type: TokenType, image: String, begin: usize, end: usize) -> Self {
         Token {
             token_type,
             image,
             begin_offset: begin,
             end_offset: end,
             next: None,
             previous: None,
         }
     }
 }

 3.4 lexer.rs.ctl (Lexer)

 use crate::arena::{Arena, TokenId};
 use crate::tokens::{Token, TokenType};
 use crate::error::ParseResult;

 pub struct Lexer<'a> {
     arena: &'a mut Arena,
     input: &'a str,
     position: usize,
     current_token: Option<TokenId>,
 }

 impl<'a> Lexer<'a> {
     pub fn new(arena: &'a mut Arena, input: &'a str) -> Self {
         Lexer {
             arena,
             input,
             position: 0,
             current_token: None,
         }
     }

     pub fn next_token(&mut self) -> ParseResult<TokenId> {
         // Lexer logic translated from grammar
         ${globals.translateLexerMethods()}
     }
 }

 3.5 parser.rs.ctl (Parser + AST Nodes)

 use crate::arena::{Arena, NodeId, TokenId};
 use crate::tokens::TokenType;
 use crate::error::{ParseError, ParseResult};
 use crate::lexer::Lexer;

 // AST Node enum containing all node types
 #[derive(Debug, Clone)]
 pub enum AstNode {
 #list grammar.nodeNames as nodeName
     ${nodeName}(${nodeName}Node),
 #endlist
 }

 // Individual node structs
 #list grammar.nodeNames as nodeName
 #[derive(Debug, Clone)]
 pub struct ${nodeName}Node {
     pub parent: Option<NodeId>,
     pub children: Vec<NodeId>,
     pub begin_token: TokenId,
     pub end_token: TokenId,
     // Node-specific fields generated from grammar
     ${globals.translateNodeFields(nodeName)}
 }
 #endlist

 // Parser struct
 pub struct Parser<'a> {
     arena: &'a mut Arena,
     lexer: Lexer<'a>,
     current_token: TokenId,
     lookahead: Vec<TokenId>,
 }

 impl<'a> Parser<'a> {
     pub fn new(arena: &'a mut Arena, input: &'a str) -> Self {
         let mut lexer = Lexer::new(arena, input);
         let current = lexer.next_token().expect("At least EOF token");

         Parser {
             arena,
             lexer,
             current_token: current,
             lookahead: Vec::new(),
         }
     }

     // Generated parsing methods
 #list grammar.parserProductions as production
     ${globals.translateParserProduction(production)}
 #endlist

     // Helper methods
     fn consume_token(&mut self) -> ParseResult<TokenId> {
         let old = self.current_token;
         self.current_token = self.lexer.next_token()?;
         Ok(old)
     }

     fn current_token_type(&self) -> TokenType {
         self.arena.get_token(self.current_token).token_type
     }
 }

 3.6 error.rs.ctl (Error Types)

 use crate::arena::TokenId;
 use crate::tokens::TokenType;
 use std::fmt;

 pub type ParseResult<T> = Result<T, ParseError>;

 #[derive(Debug, Clone)]
 pub struct ParseError {
     pub message: String,
     pub token: Option<TokenId>,
     pub expected: Option<Vec<TokenType>>,
 }

 impl ParseError {
     pub fn new(message: impl Into<String>) -> Self {
         ParseError {
             message: message.into(),
             token: None,
             expected: None,
         }
     }

     pub fn expected(types: Vec<TokenType>) -> Self {
         ParseError {
             message: format!("Expected one of: {:?}", types),
             token: None,
             expected: Some(types),
         }
     }
 }

 impl fmt::Display for ParseError {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{}", self.message)
     }
 }

 impl std::error::Error for ParseError {}

 3.7 Cargo.toml.ctl

 [package]
 name = "${settings.parserPackage?replace('.', '-')}"
 version = "0.1.0"
 edition = "2021"

 [dependencies]
 congocc-runtime = "0.1"

 [dev-dependencies]

 3.8 CommonUtils.inc.ctl

 [#-- Common Rust utility macros and functions --]

 [#function bool val]
     [#return val?string("true", "false")/]
 [/#function]

 [#macro allocateNode nodeType fields]
 let node = ${nodeType}Node {
     parent: None,
     children: Vec::new(),
     begin_token: current_token,
     end_token: current_token,
     ${fields}
 };
 let node_id = self.arena.alloc_node(AstNode::${nodeType}(node));
 [/#macro]

 4. TemplateGlobals Enhancements

 File: /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/TemplateGlobals.java

 Add Rust-specific helper methods:

 // Translate parser production to Rust method
 public String translateParserProduction(Node production) {
     if (codeLang != RUST) return "";

     StringBuilder result = new StringBuilder();
     RustTranslator translator = (RustTranslator) this.translator;

     // Generate method signature
     result.append("pub fn ");
     result.append(translator.translateIdentifier(production.getName(), METHOD));
     result.append("(&mut self) -> ParseResult<NodeId> {\n");

     // Translate method body
     translator.translateCodeBlock(production.getExpansion(), 4, result);

     result.append("}\n");
     return result.toString();
 }

 // Translate node fields from grammar
 public String translateNodeFields(String nodeName) {
     if (codeLang != RUST) return "";

     // Get node definition from grammar
     // Translate fields to Rust types
     // Return field definitions
 }

 5. Grammar Changes (Source Files)

 File: Find where CodeLang enum is defined in grammar source

 Search in: /home/rich/git/congo-rustgen/src/grammars/ or generated files

 Add: RUST to enum definition

 6. Testing Infrastructure

 6.1 Test Grammar

 File: /home/rich/git/congo-rustgen/examples/rust-test/SimpleArithmetic.ccc

 PARSER_CLASS="ArithmeticParser";
 LEXER_CLASS="ArithmeticLexer";

 Expression :
     AdditiveExpression
 ;

 AdditiveExpression :
     MultiplicativeExpression
     (
         ("+" | "-") MultiplicativeExpression
     )*
 ;

 MultiplicativeExpression :
     Primary
     (
         ("*" | "/") Primary
     )*
 ;

 Primary :
     <INTEGER>
     | "(" Expression ")"
 ;

 TOKEN :
     <INTEGER : (["0"-"9"])+ >
     | <PLUS : "+">
     | <MINUS : "-">
     | <STAR : "*">
     | <SLASH : "/">
 ;

 6.2 Test Harness

 File: /home/rich/git/congo-rustgen/examples/rust-test/test_parser.rs

 use arithmetic_parser::*;

 fn main() {
     let mut arena = Arena::new();
     let mut parser = Parser::new(&mut arena, "1 + 2 * 3");

     match parser.parse() {
         Ok(root) => println!("Parsed successfully! Root: {:?}", root),
         Err(e) => eprintln!("Parse error: {}", e),
     }
 }

 6.3 Build Test

 File: /home/rich/git/congo-rustgen/examples/rust-test/build.xml

 <target name="test">
     <exec executable="java">
         <arg line="-jar ../../congocc.jar -lang rust SimpleArithmetic.ccc"/>
     </exec>
     <exec executable="cargo" dir="generated">
         <arg line="build"/>
     </exec>
     <exec executable="cargo" dir="generated">
         <arg line="test"/>
     </exec>
 </target>

 Implementation Phases

 Phase 1: Foundation (Week 1-2)

 1. Create congocc-runtime crate with Arena, NodeId, TokenId
 2. Add RUST to CodeLang enum
 3. Create basic RustTranslator skeleton
 4. Update FilesGenerator with RUST case

 Phase 2: Templates (Week 3-5)

 1. Create lib.rs.ctl, arena.rs.ctl, tokens.rs.ctl templates
 2. Implement basic token generation
 3. Create error.rs.ctl with ParseError/ParseResult
 4. Create Cargo.toml.ctl template

 Phase 3: Lexer Generation (Week 5-6)

 1. Create lexer.rs.ctl template
 2. Implement RustTranslator.translateLexerMethods()
 3. Test lexer generation with simple grammar

 Phase 4: Parser Generation (Week 7-9)

 1. Create parser.rs.ctl template
 2. Implement RustTranslator translation for:
   - Production rules → Rust methods
   - BNF expansions → match statements
   - Lookahead → Result-based checking
 3. Implement arena-based node allocation
 4. Test with arithmetic grammar

 Phase 5: AST Node Generation (Week 10-11)

 1. Implement enum-based node type generation
 2. Translate tree-building code
 3. Handle parent/child relationships via arena
 4. Test with tree-building grammars

 Phase 6: Advanced Features (Week 11-12)

 1. Error recovery (if fault-tolerant mode enabled)
 2. Contextual predicates
 3. Context-sensitive tokenization
 4. Optimization passes

 Phase 7: Testing & Documentation (Week 12-13)

 1. Test with JSON grammar
 2. Test with Lua grammar
 3. Document Rust-specific patterns
 4. Create usage examples

 Critical Files to Create/Modify

 New Files (Java)

 - /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/rust/RustTranslator.java
 - /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/rust/RustFormatter.java (optional)

 Modified Files (Java)

 - /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/FilesGenerator.java
 - /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/Translator.java
 - /home/rich/git/congo-rustgen/src/java/org/congocc/codegen/TemplateGlobals.java
 - Grammar file containing CodeLang enum (find in src/grammars/)

 New Template Files (Rust)

 - /home/rich/git/congo-rustgen/src/templates/rust/lib.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/arena.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/tokens.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/lexer.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/parser.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/error.rs.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/Cargo.toml.ctl
 - /home/rich/git/congo-rustgen/src/templates/rust/CommonUtils.inc.ctl

 New Rust Crate

 - congocc-runtime/ (separate repository/crate)

 Key Design Decisions

 1. Arena Allocation

 Decision: Use typed-arena pattern with Vec storage
 Rationale: Zero-cost, cache-friendly, no Rc/RefCell overhead
 Trade-off: Requires lifetime annotations, more complex than GC

 2. Enum-Based Nodes

 Decision: Single AstNode enum containing all node types
 Rationale: Idiomatic Rust, enables exhaustive pattern matching
 Trade-off: Larger enum size, but eliminates vtable overhead

 3. Result-Based Errors

 Decision: All parser methods return Result<T, ParseError>
 Rationale: Idiomatic Rust error handling
 Trade-off: More verbose than exceptions, but compile-time checked

 4. Index-Based References

 Decision: Use NodeId/TokenId instead of pointers
 Rationale: Avoids lifetime complexity, enables serialization
 Trade-off: Extra indirection, but negligible performance impact

 5. Runtime Support Crate

 Decision: Create congocc-runtime with common utilities
 Rationale: Share code, version independently, reduce generated code size
 Trade-off: Extra dependency, but standard practice for parser generators

 6. Cross-Language Translation

 Decision: Translate Java semantics to Rust arena patterns
 Rationale: Maintain grammar compatibility
 Trade-off: Generated code less manual-looking, but automatic

 Success Criteria

 1. ✅ Existing Java/Python/C# grammars generate valid Rust code
 2. ✅ Generated parsers compile with cargo build
 3. ✅ Generated parsers pass tests with cargo test
 4. ✅ JSON example grammar works in Rust
 5. ✅ No unsafe code in generated parsers
 6. ✅ No runtime panics (all errors are Result-based)
 7. ✅ Performance comparable to hand-written Rust parsers

 Timeline

 Total Estimated Effort: 12-13 weeks (full-time)

 - Weeks 1-2: Foundation
 - Weeks 3-5: Templates
 - Weeks 5-6: Lexer
 - Weeks 7-9: Parser
 - Weeks 10-11: AST Nodes
 - Weeks 11-12: Advanced Features
 - Week 12-13: Testing & Docs

 Next Steps After Plan Approval

 1. Create congocc-runtime repository and basic Arena implementation
 2. Find and update CodeLang enum definition
 3. Create RustTranslator.java skeleton
 4. Add RUST case to FilesGenerator.java
 5. Create first template (lib.rs.ctl) and test generation
╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌

 Would you like to proceed?

 ❯ 1. Yes, and auto-accept edits
   2. Yes, and manually approve edits
   3. Type here to tell Claude what to change

 ctrl-g to edit in VS Code

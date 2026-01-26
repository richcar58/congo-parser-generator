# Rust Parser Generation Learnings

This document captures key learnings from implementing arena-based Rust parsers for CongoCC.

## Arena Allocation Pattern

### Why Arena Allocation?

Rust's ownership system makes traditional tree structures challenging. Without arena allocation:
- Parent-child relationships require `Rc<RefCell<T>>` or unsafe code
- Tree traversal becomes complex with borrowing rules
- Memory fragmentation from individual node allocations

Arena allocation solves these problems:
- All nodes live in a single `Vec<AstNode>` with stable indices
- `NodeId` and `TokenId` are simple `usize` wrappers - Copy, cheap to pass around
- Parent/child relationships are just indices, no lifetime issues
- Entire tree deallocated at once when Arena is dropped

### Core Arena Structure

```rust
pub struct NodeId(pub usize);  // Type-safe index
pub struct TokenId(pub usize);

pub struct Arena {
    nodes: Vec<AstNode>,
    tokens: Vec<Token>,
}

impl Arena {
    pub fn alloc_node(&mut self, node: AstNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }

    pub fn get_node(&self, id: NodeId) -> &AstNode { &self.nodes[id.0] }
    pub fn get_node_mut(&mut self, id: NodeId) -> &mut AstNode { &mut self.nodes[id.0] }
}
```

### Node Structure Pattern

Each AST node type follows this pattern:

```rust
pub struct ExpressionNode {
    pub parent: Option<NodeId>,      // Optional parent link
    pub children: Vec<NodeId>,       // Child node indices
    pub begin_token: TokenId,        // First token span
    pub end_token: TokenId,          // Last token span
    // ... production-specific fields
}
```

## Parser Integration

### Parser Struct with Arena

```rust
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    lookahead: Vec<Token>,
    arena: Arena,                    // Owns all nodes
    current_token_id: Option<TokenId>,
}

impl Parser {
    pub fn arena(&self) -> &Arena { &self.arena }
    pub fn arena_mut(&mut self) -> &mut Arena { &mut self.arena }
}
```

### Parse Method Pattern

Every `parse_*` method returns `ParseResult<NodeId>`:

```rust
fn parse_expression(&mut self) -> ParseResult<NodeId> {
    let begin_token = self.alloc_current_token();

    // Parse children
    let child = self.parse_additive_expression()?;

    // Allocate node
    let end_token = self.current_token_id.unwrap_or(begin_token);
    let mut node = ExpressionNode::new(begin_token, end_token);
    node.children.push(child);

    let node_id = self.arena.alloc_node(AstNode::Expression(node));

    // Set parent relationship
    self.set_parent(child, node_id);

    Ok(node_id)
}
```

### Token Allocation

Tokens are allocated to the arena for span tracking:

```rust
fn alloc_current_token(&mut self) -> TokenId {
    let token_id = self.arena.alloc_token(self.current_token.clone());
    self.current_token_id = Some(token_id);
    token_id
}
```

## Rust-Specific Considerations

### Pattern Matching Limitations

In Rust, OR patterns (`|`) cannot bind different types to the same variable:

```rust
// WRONG - different types can't share binding
match arena.get_node(id) {
    AstNode::Expression(node) |
    AstNode::Term(node) => { /* node has different types */ }
}

// CORRECT - handle each separately
match arena.get_node(id) {
    AstNode::Expression(node) => process(&node.children),
    AstNode::Term(node) => process(&node.children),
}
```

### Error Handling

Use `Result` types consistently:

```rust
pub type ParseResult<T> = Result<T, ParseError>;

pub struct ParseError {
    pub message: String,
    pub position: Option<usize>,
    pub line: Option<usize>,
    pub column: Option<usize>,
}
```

### Consuming vs Borrowing

The lexer consumes input, parser borrows tokens:
- `Lexer::new(input: String)` - takes ownership
- `Parser::new(input: String)` - creates lexer internally
- Arena methods use `&self` for reads, `&mut self` for allocations

## Template Considerations

### Generated Code Structure

Templates should generate:
1. `arena.rs` - Arena type with node allocation
2. `tokens.rs` - Token types and TokenType enum
3. `lexer.rs` - Tokenizer implementation
4. `parser.rs` - Parser with arena integration
5. `error.rs` - Error types
6. `lib.rs` - Public API re-exports

### Naming Conventions

- Node types: `{ProductionName}Node` (e.g., `ExpressionNode`)
- Parse methods: `parse_{production_name}` (e.g., `parse_expression`)
- Token types: Use SCREAMING_SNAKE_CASE (e.g., `INTEGER_LITERAL`)

## Testing Strategy

### Positive Tests (Valid Input)

```rust
#[test]
fn test_simple_expression() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    // Verify node type
    assert!(matches!(parser.arena().get_node(root), AstNode::Expression(_)));
}
```

### Negative Tests (Invalid Input)

```rust
#[test]
fn test_missing_operand() {
    let mut parser = Parser::new("1 +".to_string()).unwrap();
    assert!(parser.parse().is_err());
}
```

### AST Structure Tests

```rust
#[test]
fn test_ast_structure() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();

    match parser.arena().get_node(root) {
        AstNode::Expression(node) => {
            assert_eq!(node.children.len(), 1);
            // Verify child structure...
        }
        _ => panic!("Expected Expression"),
    }
}
```

### Pretty-Print Tests

```rust
#[test]
fn test_pretty_print() {
    let mut parser = Parser::new("1 + 2".to_string()).unwrap();
    let root = parser.parse().unwrap();
    let output = parser.arena().pretty_print(root, 0);

    assert!(output.contains("Expression"));
    assert!(output.contains("AdditiveExpression"));
}
```

## File Organization

```
examples/rust-test/
├── arithmetic/
│   ├── Cargo.toml
│   ├── SimpleArithmetic.ccc
│   ├── src/
│   │   ├── lib.rs
│   │   ├── arena.rs
│   │   ├── tokens.rs
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   └── error.rs
│   └── tests/
│       └── integration_test.rs
└── sqlexpr/
    ├── Cargo.toml
    ├── SqlExpr.ccc
    ├── src/
    │   └── (same structure)
    └── tests/
        └── integration_test.rs
```

## Enhanced AST Capabilities

### Storing Original Input

Store the original input string in the Parser for AST processing and pretty-printing:

```rust
pub struct Parser {
    // ... other fields
    input: String,  // Original input string
}

impl Parser {
    pub fn new(input: String) -> ParseResult<Self> {
        let mut lexer = Lexer::new(input.clone());  // Clone before moving
        // ...
        Ok(Parser { /* ... */ input })
    }

    pub fn input(&self) -> &str { &self.input }
}
```

### Operator Storage Pattern

For expression nodes that have operators (e.g., `AdditiveExpression`, `MultiplicativeExpression`), store operators in a parallel vector:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditiveOp {
    Add,  // +
    Sub,  // -
}

pub struct AdditiveExpressionNode {
    pub children: Vec<NodeId>,
    pub operators: Vec<AdditiveOp>,  // operators[i] is between children[i] and children[i+1]
    // ... other fields
}
```

**Key insight**: For N operands, there are N-1 operators. The operators vector stores these in order.

Example: `1 + 2 - 3` results in:
- `children: [node1, node2, node3]`
- `operators: [Add, Sub]`

### Accessor Methods

Provide convenient accessor methods that work with the Arena:

```rust
impl AdditiveExpressionNode {
    /// Get the left operand (first child)
    pub fn left<'a>(&self, arena: &'a Arena) -> &'a AstNode {
        arena.get_node(self.children[0])
    }

    /// Get the right operand (second child for binary case)
    pub fn right<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.get(1).map(|id| arena.get_node(*id))
    }

    /// Get first operator (for binary expressions)
    pub fn first_op(&self) -> Option<AdditiveOp> {
        self.operators.first().copied()
    }
}
```

**Design choice**: Accessor methods take `&Arena` as parameter to avoid lifetime complexity. This keeps node structs simple while enabling convenient access.

### Pass-Through Node Detection

A "pass-through" node is one with no semantic value except a single child. These clutter pretty-print output:

```rust
fn is_passthrough(&self, node_id: NodeId) -> bool {
    match self.get_node(node_id) {
        AstNode::Expression(n) => n.children.len() == 1,
        AstNode::AdditiveExpression(n) => n.children.len() == 1 && n.operators.is_empty(),
        AstNode::MultiplicativeExpression(n) => n.children.len() == 1 && n.operators.is_empty(),
        _ => false,
    }
}
```

In pretty_print, skip pass-through nodes and recurse directly to their child:

```rust
if self.is_passthrough(node_id) {
    if let Some(child) = self.first_child(node_id) {
        self.pretty_print_impl(child, indent, result);  // Same indent level
    }
    return;
}
```

### Enhanced Pretty-Print

The improved pretty_print shows:
1. Original input on the first line
2. Operators/values in brackets for nodes that have them
3. Collapsed pass-through nodes for cleaner output

```rust
pub fn pretty_print(&self, root: NodeId, indent: usize, input: &str) -> String {
    let mut result = format!("AST for: \"{}\"\n", input);
    self.pretty_print_impl(root, indent, &mut result);
    result
}
```

**Before** (cluttered):
```
Expression
  AdditiveExpression
    MultiplicativeExpression
      Primary("1")
    MultiplicativeExpression
      Primary("2")
```

**After** (enhanced):
```
AST for: "1 + 2"
AdditiveExpression [+]
  Primary("1")
  Primary("2")
```

### Typed Arena Accessors

Add typed getter methods to Arena for convenience:

```rust
impl Arena {
    pub fn get_additive(&self, id: NodeId) -> Option<&AdditiveExpressionNode> {
        match self.get_node(id) {
            AstNode::AdditiveExpression(node) => Some(node),
            _ => None,
        }
    }
    // Similar methods for other node types...
}
```

## Summary

Key principles for Rust parser generation:
1. **Arena allocation** eliminates lifetime complexity for tree structures
2. **Type-safe indices** (`NodeId`, `TokenId`) prevent mixing up node/token references
3. **Result types** provide clean error propagation
4. **Explicit parent setting** after node allocation maintains tree relationships
5. **Token span tracking** (`begin_token`, `end_token`) enables source location reporting
6. **Original input storage** enables AST processing to access source text
7. **Operator storage** preserves semantic information that would otherwise be lost
8. **Accessor methods** provide node-level convenience while avoiding lifetime issues
9. **Pass-through detection** enables cleaner pretty-print output

---

## Backporting AST Enhancements to Code Generator

### Template Architecture

CongoCC uses the CTL (Congo Template Language) template engine for code generation. Rust templates are in `src/templates/rust/`:

```
src/templates/rust/
├── arena.rs.ctl      # Arena allocator, node structs, pretty_print
├── parser.rs.ctl     # Parser struct, parse methods
├── tokens.rs.ctl     # Token types and TokenType enum
├── lexer.rs.ctl      # Lexer implementation
├── error.rs.ctl      # Error types
├── lib.rs.ctl        # Module re-exports
└── Cargo.toml.ctl    # Rust manifest
```

### CTL Template Syntax

Templates use Freemarker-like syntax:

```
[#if condition]
  // Conditional content
[/#if]

[#list grammar.parserProductions as production]
  // Iterate over productions
  ${production.name?cap_first}  // Capitalize first letter
[/#list]

${globals::translateIdentifier(name)}  // Call translator function
```

### Template Variables Available

- `grammar` - Grammar object with productions, lexer data
- `settings` - AppSettings with parser configuration
- `globals` - TemplateGlobals with translation methods
- `lexerData` - Lexical analysis structures
- `generated_by` - Generator identification string

### RustTranslator.java

The `RustTranslator` class in `src/java/org/congocc/codegen/rust/` handles language-specific translations:

- Type translations: `boolean` → `bool`, `List` → `Vec<NodeId>`
- Identifier translations: `null` → `None`, camelCase → snake_case
- Method name conversions: `toString` → `to_string`

### Categorizing Enhancements for Backporting

**Generic Enhancements** (apply to all parsers):
- Input storage in Parser (`input` field, `input()` accessor)
- Arena ownership in Parser (`arena` field, `arena()` accessor)
- Basic `pretty_print()` method structure
- Token allocation helpers
- Pass-through detection framework

**Grammar-Aware Enhancements** (require grammar analysis):
- Operator enum generation (AdditiveOp, ComparisonOp, etc.)
- Operator storage in node structs (`operators: Vec<OpType>`)
- Accessor methods (left(), right(), first_op())
- Production-specific pretty_print formatting

### Strategies for Grammar-Aware Generation

**Option 1: Naming Convention** (simplest)
```
// Productions ending in "Expression" get operator support
[#if production.name?ends_with("Expression")]
    pub operators: Vec<${production.name?cap_first}Op>,
[/#if]
```

**Option 2: Grammar Annotations** (most flexible)
```
// In .ccc file:
AdditiveExpression #operator("+", "-") :
    MultiplicativeExpression ( (<PLUS> | <MINUS>) MultiplicativeExpression )*
;
```

**Option 3: Pattern Detection** (most complex)
Analyze expansion tree to detect `A (op A)*` patterns automatically.

### Pretty Print Output Format

The standard format for AST pretty printing:

```
AST: "original input"
  NodeType [operator]
    ChildNode
    ChildNode
```

Key features:
1. First line shows original input
2. Operators displayed in brackets: `[+]`, `[=]`, `[AND x2]`
3. Pass-through nodes collapsed (single child, no semantic value)
4. Consistent 2-space indentation

### Documentation Requirements

All generated code should include documentation for:
- Enum variants (AstNode variants, operator enums)
- Struct fields (parent, children, begin_token, end_token, operators)
- Public methods (new(), left(), right(), op(), value())
- Module-level documentation in lib.rs

### Testing Generated Code

After template changes, verify:
1. Generated code compiles: `cargo build`
2. Tests pass: `cargo test`
3. Pretty print output format correct
4. Operator storage works for expression nodes
5. No clippy warnings for missing documentation

---

## Phase 2: Generic Operator Storage Implementation

### Design Decision: TokenId-based Operators

Rather than generating typed operator enums (which requires grammar analysis), Phase 2 uses a generic approach:

```rust
pub struct ExpressionNode {
    pub children: Vec<NodeId>,
    pub operators: Vec<TokenId>,  // Stores token IDs, not typed enums
    // ...
}
```

**Advantages:**
- Works without grammar analysis
- Operator token images accessible via `arena.get_token(tid).image`
- Same pattern applies to all node types
- Can be enhanced later with typed enums

**Trade-offs:**
- No compile-time type safety for operator matching
- Users must inspect token images rather than match on enum variants

### Pass-Through Detection with Operators

A node is pass-through only if it has a single child AND no operators:

```rust
fn is_passthrough(&self, node_id: NodeId) -> bool {
    match self.get_node(node_id) {
        AstNode::Expression(n) => n.children.len() == 1 && n.operators.is_empty(),
        // ...
    }
}
```

This ensures nodes like `1 + 2` (which has operators) are not collapsed.

### Accessor Methods Pattern

All node types get the same accessor methods:

```rust
impl ExpressionNode {
    /// Get the left operand (first child)
    pub fn left<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.first().map(|id| arena.get_node(*id))
    }

    /// Get the right operand (second child)
    pub fn right<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.get(1).map(|id| arena.get_node(*id))
    }

    /// Get first operator token
    pub fn first_op(&self) -> Option<TokenId> {
        self.operators.first().copied()
    }

    /// Get operator at index
    pub fn op(&self, index: usize) -> Option<TokenId> {
        self.operators.get(index).copied()
    }

    /// Get token image of first token (for leaf nodes)
    pub fn value<'a>(&self, arena: &'a Arena) -> &'a str {
        &arena.get_token(self.begin_token).image
    }
}
```

### Enhanced Pretty Print with Operators

Pretty print shows operator token images when present:

```rust
if node.operators.is_empty() {
    result.push_str(&format!("{}NodeName\n", indent_str));
} else {
    let ops: Vec<&str> = node.operators.iter()
        .map(|tid| self.get_token(*tid).image.as_str())
        .collect();
    result.push_str(&format!("{}NodeName [{}]\n", indent_str, ops.join(", ")));
}
```

Example output:
```
AST: "1 + 2 * 3"
AdditiveExpression [+]
  Primary
  MultiplicativeExpression [*]
    Primary
    Primary
```

### Template Implementation

The key template changes in `arena.rs.ctl`:

1. Added `operators` field to node struct:
```
pub operators: Vec<TokenId>,
```

2. Initialize in constructor:
```
operators: Vec::new(),
```

3. Updated `is_passthrough` to check `n.operators.is_empty()`

4. Updated `pretty_print_impl` to display operator images

---

## Phase 3: Leaf Node Value Display

### Problem

Without showing leaf node values, pretty_print output like this is not useful for debugging:

```
AST: "1 + 2"
AdditiveExpression [+]
  Primary
  Primary
```

### Solution

Detect leaf nodes (nodes with no children) and display their token value:

```rust
if node.children.is_empty() {
    // Leaf node - show the token value
    let value = &self.get_token(node.begin_token).image;
    result.push_str(&format!("{}NodeName(\"{}\")\n", indent_str, value));
} else if node.operators.is_empty() {
    // Internal node without operators
    result.push_str(&format!("{}NodeName\n", indent_str));
    // recurse to children...
} else {
    // Internal node with operators
    // show operators and recurse...
}
```

### Result

Pretty print now shows useful output:

```
AST: "1 + 2"
AdditiveExpression [+]
  Primary("1")
  Primary("2")
```

### Template Change

In `arena.rs.ctl`, the `pretty_print_impl` match arm now has three cases:
1. **Leaf node** (`children.is_empty()`) - show `NodeName("value")`
2. **Internal node without operators** - show `NodeName` and recurse
3. **Internal node with operators** - show `NodeName [ops]` and recurse

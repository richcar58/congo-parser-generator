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

## Summary

Key principles for Rust parser generation:
1. **Arena allocation** eliminates lifetime complexity for tree structures
2. **Type-safe indices** (`NodeId`, `TokenId`) prevent mixing up node/token references
3. **Result types** provide clean error propagation
4. **Explicit parent setting** after node allocation maintains tree relationships
5. **Token span tracking** (`begin_token`, `end_token`) enables source location reporting

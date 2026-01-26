# Plan: Backporting AST Enhancements to Rust Code Generator

## Executive Summary

This plan outlines how to incorporate the AST enhancements developed in the `rust-test/arithmetic` and `rust-test/sqlexpr` examples into the CongoCC Rust code generation templates, so all future generated Rust parsers will include these capabilities.

## Current State vs. Enhanced State

### Current Templates (src/templates/rust/)

| File | Current State |
|------|--------------|
| `arena.rs.ctl` | Basic arena with NodeId/TokenId, simple node structs (parent, children, begin/end tokens) |
| `parser.rs.ctl` | Basic parser struct with lexer, no arena integration, stub parse methods |
| `lib.rs.ctl` | Basic module exports, no operator enums |

### Enhancements to Backport

| Enhancement | Description | Template Impact |
|-------------|-------------|-----------------|
| **Input Storage** | Store original input in Parser, accessible via `input()` | `parser.rs.ctl` |
| **Arena Integration** | Parser owns Arena, accessible via `arena()` | `parser.rs.ctl` |
| **Pretty Print** | `pretty_print(root, indent, input)` with pass-through detection | `arena.rs.ctl` |
| **Operator Enums** | Grammar-specific operator types (AdditiveOp, ComparisonOp, etc.) | `arena.rs.ctl` |
| **Accessor Methods** | `left()`, `right()`, `op()`, `first_op()`, `value()` | `arena.rs.ctl` |
| **Operator Storage** | `operators: Vec<OpType>` field in expression nodes | `arena.rs.ctl` |
| **Full Documentation** | Doc comments for all public items | All templates |

---

## Phase 1: Generic Enhancements (No Grammar Analysis Required)

These enhancements apply to all parsers regardless of grammar structure.

### 1.1 Parser Input Storage (`parser.rs.ctl`)

**Current:**
```rust
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    lookahead: Vec<Token>,
}

pub fn new(input: String) -> ParseResult<Self> {
    let mut lexer = Lexer::new(input);
    // ...
}
```

**Enhanced:**
```rust
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    lookahead: Vec<Token>,
    arena: Arena,
    current_token_id: Option<TokenId>,
    input: String,  // ADD: Store original input
}

pub fn new(input: String) -> ParseResult<Self> {
    let mut lexer = Lexer::new(input.clone());  // Clone before moving
    // ...
    Ok(Parser {
        // ...
        arena: Arena::new(),
        current_token_id: None,
        input,  // Store original
    })
}

/// Get the original input string
pub fn input(&self) -> &str { &self.input }

/// Get a reference to the arena
pub fn arena(&self) -> &Arena { &self.arena }
```

### 1.2 Basic Pretty Print (`arena.rs.ctl`)

Add a generic pretty_print that works for any grammar:

```rust
impl Arena {
    /// Pretty print the AST starting from the given node
    pub fn pretty_print(&self, root: NodeId, indent: usize, input: &str) -> String {
        let mut result = format!("AST: \"{}\"\n", input);
        self.pretty_print_impl(root, indent + 1, &mut result);
        result
    }

    fn pretty_print_impl(&self, node_id: NodeId, indent: usize, result: &mut String) {
        let indent_str = "  ".repeat(indent);
        match self.get_node(node_id) {
[#list grammar.parserProductions as production]
            AstNode::${production.name?cap_first}(node) => {
                result.push_str(&format!("{}${production.name?cap_first}\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
[/#list]
        }
    }
}
```

### 1.3 Token Allocation Helper (`parser.rs.ctl`)

```rust
/// Allocate current token to the arena and track its ID
fn alloc_current_token(&mut self) -> TokenId {
    let token_id = self.arena.alloc_token(self.current_token.clone());
    self.current_token_id = Some(token_id);
    token_id
}

/// Set parent relationship for a child node
fn set_parent(&mut self, child: NodeId, parent: NodeId) {
    // Implementation varies by node type - use match on AstNode
}
```

### 1.4 Update lib.rs.ctl Doctest

```rust
//! # Example
//!
//! ```no_run
//! use ${settings.parserPackage?replace(".", "_")}::*;
//!
//! fn main() -> Result<(), ParseError> {
//!     let input = "your input here".to_string();
//!     let mut parser = Parser::new(input)?;
//!     let root = parser.parse()?;
//!     println!("{}", parser.arena().pretty_print(root, 0, parser.input()));
//!     Ok(())
//! }
//! ```
```

---

## Phase 2: Grammar-Aware Enhancements (Requires Grammar Analysis)

These enhancements require analyzing the grammar to detect patterns.

### 2.1 Operator Detection Strategy

**Problem:** We need to identify which productions represent binary/n-ary expressions with operators.

**Solution Options:**

#### Option A: Grammar Annotations (Recommended)
Add annotations in the `.ccc` grammar file:

```
// In SqlExpr.ccc
AdditiveExpression #operator("+", "-") :
    MultiplicativeExpression ( (<PLUS> | <MINUS>) MultiplicativeExpression )*
;
```

The `#operator` annotation tells the generator to:
1. Create an operator enum (`AdditiveOp`)
2. Add `operators: Vec<AdditiveOp>` to the node struct
3. Generate accessor methods

#### Option B: Pattern Detection (More Complex)
Analyze the grammar structure to detect patterns like:
```
A : B ( (op1 | op2) B )*
```

This requires:
- Walking the expansion tree
- Detecting choice nodes containing only token references
- Detecting repetition with alternating operator/operand

#### Option C: Naming Convention
Use naming conventions to trigger operator generation:
- Productions ending in `Expression` get operator support
- Productions ending in `List` get list helpers

**Recommendation:** Start with Option C (naming convention) as it requires no grammar changes, then add Option A for explicit control.

### 2.2 Operator Enum Generation (`arena.rs.ctl`)

When a production matches the operator pattern:

```rust
[#-- Detect if production has operators based on naming convention --]
[#assign hasOperators = production.name?ends_with("Expression") && production.expansion??]

[#if hasOperators]
/// Operator for ${production.name}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ${production.name?cap_first}Op {
    // TODO: Extract operator tokens from grammar
    // This requires analyzing production.expansion to find operator tokens
}
[/#if]
```

### 2.3 Node Struct with Operators

```rust
pub struct ${production.name?cap_first}Node {
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
[#if hasOperators]
    /// Operators between children: operators[i] is between children[i] and children[i+1]
    pub operators: Vec<${production.name?cap_first}Op>,
[/#if]
    pub begin_token: TokenId,
    pub end_token: TokenId,
}
```

### 2.4 Accessor Methods

```rust
impl ${production.name?cap_first}Node {
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self { ... }

[#if hasOperators]
    /// Get the left operand (first child)
    pub fn left<'a>(&self, arena: &'a Arena) -> &'a AstNode {
        arena.get_node(self.children[0])
    }

    /// Get the right operand (second child for binary case)
    pub fn right<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.get(1).map(|id| arena.get_node(*id))
    }

    /// Get first operator
    pub fn first_op(&self) -> Option<${production.name?cap_first}Op> {
        self.operators.first().copied()
    }

    /// Get operator at index
    pub fn op(&self, index: usize) -> Option<${production.name?cap_first}Op> {
        self.operators.get(index).copied()
    }
[/#if]
}
```

### 2.5 Enhanced Pretty Print with Operators

```rust
AstNode::${production.name?cap_first}(node) => {
[#if hasOperators]
    let ops: Vec<&str> = node.operators.iter()
        .map(|op| match op {
            // TODO: Generate match arms from operator enum
        })
        .collect();
    if ops.is_empty() {
        result.push_str(&format!("{}${production.name?cap_first}\n", indent_str));
    } else {
        result.push_str(&format!("{}${production.name?cap_first} [{}]\n", indent_str, ops.join(", ")));
    }
[#else]
    result.push_str(&format!("{}${production.name?cap_first}\n", indent_str));
[/#if]
    for child in &node.children {
        self.pretty_print_impl(*child, indent + 1, result);
    }
}
```

### 2.6 Pass-Through Detection

```rust
fn is_passthrough(&self, node_id: NodeId) -> bool {
    match self.get_node(node_id) {
[#list grammar.parserProductions as production]
        AstNode::${production.name?cap_first}(n) => {
[#if hasOperators]
            n.children.len() == 1 && n.operators.is_empty()
[#else]
            n.children.len() == 1
[/#if]
        }
[/#list]
    }
}
```

---

## Phase 3: RustTranslator.java Modifications

The `RustTranslator` class may need updates to support new translation patterns:

### 3.1 Operator Token Translation

Add method to translate token names to Rust enum variants:

```java
public String translateOperatorVariant(String tokenName) {
    // PLUS -> Add, MINUS -> Sub, STAR -> Mul, SLASH -> Div
    // EQ -> Eq, NE -> Ne, LT -> Lt, GT -> Gt, etc.
}
```

### 3.2 Grammar Analysis Helper

Add utility to detect operator patterns in productions:

```java
public boolean isOperatorProduction(BNFProduction production) {
    // Analyze expansion to detect (A (op A)* ) pattern
}

public List<String> extractOperatorTokens(BNFProduction production) {
    // Return list of operator token names from the production
}
```

---

## Implementation Order

### Step 1: Generic Enhancements (Low Risk)
1. Add `input` field to Parser struct template
2. Add `arena` field and accessor to Parser
3. Add `input()` accessor method
4. Update `new()` to store input
5. Add basic `pretty_print()` to Arena
6. Update lib.rs doctest

**Estimated effort:** 1-2 hours
**Risk:** Low - additive changes only

### Step 2: Pass-Through Detection (Medium Risk)
1. Add `is_passthrough()` method to Arena
2. Add `first_child()` helper
3. Update `pretty_print_impl()` to skip pass-through nodes

**Estimated effort:** 1-2 hours
**Risk:** Medium - changes pretty_print behavior

### Step 3: Operator Support (Higher Complexity)
1. Add naming convention detection in template
2. Generate operator enums for matching productions
3. Add `operators` field to node structs
4. Generate accessor methods
5. Update pretty_print to show operators

**Estimated effort:** 4-8 hours
**Risk:** Medium - requires careful template logic

### Step 4: Grammar Annotations (Future)
1. Define annotation syntax in CongoCC grammar
2. Parse annotations in core/BNFProduction.java
3. Expose annotations to templates via TemplateGlobals
4. Update templates to use annotations

**Estimated effort:** 8-16 hours
**Risk:** Higher - touches core grammar parsing

---

## Testing Strategy

### Unit Tests
1. Generate parser from arithmetic grammar, verify enhancements present
2. Generate parser from sqlexpr grammar, verify enhancements present
3. Compile generated code, run existing tests

### Integration Tests
1. Add test cases to `examples/rust-test/` build
2. Verify pretty_print output format
3. Verify operator storage works

### Regression Tests
1. Regenerate existing Rust parsers
2. Verify no compilation errors
3. Verify test suites still pass

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/templates/rust/parser.rs.ctl` | Add input/arena fields, accessors, token allocation |
| `src/templates/rust/arena.rs.ctl` | Add pretty_print, pass-through detection, operator support |
| `src/templates/rust/lib.rs.ctl` | Export operator enums, update doctest |
| `src/java/org/congocc/codegen/rust/RustTranslator.java` | Add operator translation helpers |
| `src/java/org/congocc/codegen/TemplateGlobals.java` | Add grammar analysis utilities (optional) |

---

## Success Criteria

1. ✅ Generated parsers compile without errors
2. ✅ `parser.input()` returns original input string
3. ✅ `parser.arena()` provides access to the arena
4. ✅ `pretty_print()` shows "AST: input" on first line
5. ✅ Pass-through nodes are collapsed in pretty_print output
6. ✅ Expression nodes have operator storage (where applicable)
7. ✅ Accessor methods available on expression nodes
8. ✅ All public items have documentation
9. ✅ Existing test suites pass after regeneration

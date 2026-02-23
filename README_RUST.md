# Rust Code Generation

CongoCC can generate idiomatic Rust parsers with modern features including arena allocation, type-safe indices, and comprehensive error handling.

## Generating a Rust Parser

To generate a Rust parser from your grammar file:

```bash
java -jar congocc.jar -lang rust YourGrammar.ccc
```

The `-lang rust` option tells CongoCC to generate Rust code instead of the default Java.

## Specifying Output Directory

By default, CongoCC generates code in the same directory as your grammar file. To specify a different output directory:

```bash
java -jar congocc.jar -lang rust -d /path/to/output YourGrammar.ccc
```

Or use the `OUTPUT_DIRECTORY` option in your grammar file:

```
options {
    OUTPUT_DIRECTORY = "src/generated";
}
```

## Generated Files

CongoCC generates a complete Rust crate with the following structure:

- **`lib.rs`** - Module root with public API
- **`arena.rs`** - Arena allocator for AST nodes and tokens
- **`tokens.rs`** - Token type definitions and enums
- **`lexer.rs`** - Lexical analyzer (tokenizer)
- **`parser.rs`** - Recursive descent parser (scaffolding — see note below)
- **`error.rs`** - Error types with location tracking
- **`visitor.rs`** - Closure-based depth-first AST visitor
- **`Cargo.toml`** - Rust package manifest

## Compiling the Generated Parser

The generated code is a standard Rust library crate. To compile it:

1. Navigate to the output directory:
   ```bash
   cd /path/to/output
   ```

2. Build with Cargo:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run tests (showing output):
   ```bash
   cargo test -- --nocapture
   ```

## Integrating into Your Application

### Adding as a Dependency

If your parser is in a separate directory, add it to your `Cargo.toml`:

```toml
[dependencies]
my_parser = { path = "../path/to/generated/parser" }
```

Or publish to crates.io and reference by version:

```toml
[dependencies]
my_parser = "0.1.0"
```

### Basic Usage

```rust
use my_parser::{Parser, ParseError};

fn main() -> Result<(), ParseError> {
    // Create parser with input string
    let input = "your input text here".to_string();
    let mut parser = Parser::new(input)?;

    // Parse the input
    parser.parse()?;

    println!("Parsing successful!");
    Ok(())
}
```

### Working with AST Nodes (Arena-based)

The generated parser uses arena allocation for memory efficiency:

```rust
use my_parser::{Parser, Arena, NodeId, AstNode};

fn main() -> Result<(), ParseError> {
    let input = "your input".to_string();
    let mut parser = Parser::new(input)?;

    // Parse returns root node ID
    let root_id = parser.parse()?;

    // Access nodes through the arena
    let arena = parser.arena();
    match arena.get_node(root_id) {
        AstNode::Expression(expr) => {
            // Work with expression node
            for child_id in &expr.children {
                let child = arena.get_node(*child_id);
                // Process child nodes...
            }
        }
        _ => {}
    }

    Ok(())
}
```

### Error Handling with Location Information

The generated parser provides detailed error messages with location information:

```rust
use my_parser::Parser;

fn parse_input(input: String) {
    match Parser::new(input) {
        Ok(mut parser) => {
            match parser.parse() {
                Ok(_) => println!("Success!"),
                Err(e) => eprintln!("Parse error: {}", e),
                // Error message includes line/column info:
                // "Parse error at position 42: Expected INTEGER, found PLUS '+'"
            }
        }
        Err(e) => eprintln!("Lexer error: {}", e),
    }
}
```

## Dependencies and Features

The generated Rust parser has **zero runtime dependencies** by default. However, you can enable optional features:

### Optional Serde Support

Generated parsers include built-in, optional [serde](https://serde.rs) support. All AST types (`AstNode`, node structs, operator enums), tokens (`Token`, `TokenType`), indices (`NodeId`, `TokenId`), the `Arena` itself, and `ParseError` derive `Serialize` and `Deserialize` when the feature is enabled.

To use it, enable the `serde` feature in the consuming crate's `Cargo.toml`:

```toml
[dependencies]
my_parser = { path = "../path/to/generated/parser", features = ["serde"] }
```

Or build/test directly with the feature flag:

```bash
cargo build --features serde
cargo test --features serde
```

This enables full round-trip serialization to JSON (via `serde_json`), MessagePack, CBOR, or any other serde-compatible format:

```rust
use my_parser::{Parser, Arena, NodeId};

let mut parser = Parser::new("1 + 2".to_string())?;
let root = parser.parse()?;

// Serialize the entire AST arena to JSON
let json = serde_json::to_string_pretty(parser.arena())?;

// Deserialize back
let arena: Arena = serde_json::from_str(&json)?;
```

The feature is zero-cost when disabled — all serde annotations use `#[cfg_attr(feature = "serde", ...)]` and are completely compiled away without the feature flag.

### Memory Characteristics

- **Arena allocation**: All AST nodes are stored in a contiguous memory arena
- **Type-safe indices**: `NodeId` and `TokenId` provide safe references without lifetimes
- **Zero-cost abstractions**: No `Rc<RefCell<>>` overhead
- **Cache-friendly**: Contiguous memory layout improves performance

### Safety Guarantees

- **No unsafe code**: Generated parsers use only safe Rust
- **Result-based errors**: All parsing operations return `Result<T, ParseError>`
- **Compile-time checks**: Rust's type system catches errors at compile time

## Rust Examples

The `examples/rust-test/` directory contains two working Rust parser examples with comprehensive integration tests:

### Arithmetic Parser [(link)](examples/rust-test/arithmetic/README.md)

A simple arithmetic expression parser supporting `+`, `-`, `*`, `/`, parentheses, and integers.

```bash
# Run the existing tests (45 tests)
cd examples/rust-test/arithmetic
cargo test

# Regenerate from the grammar (preserves hand-written parser.rs)
java -jar congocc.jar -lang rust SimpleArithmetic.ccc
cargo test
```

### SQL Expression Parser [(link)](examples/rust-test/sqlexpr/README.md)

A SQL filter expression parser supporting boolean operators (`AND`, `OR`, `NOT`), comparisons (`=`, `<>`, `!=`, `<`, `>`, `<=`, `>=`), `LIKE`, `IN`, `BETWEEN`, `IS NULL`/`IS NOT NULL`, arithmetic, and literals.

```bash
# Run the existing tests (65 tests, including visitor tests)
cd examples/rust-test/sqlexpr
cargo test

# Regenerate from the grammar (preserves hand-written parser.rs)
java -jar congocc.jar -lang rust SqlExpr.ccc
cargo test
```

### Regeneration Workflow

Both examples have hand-written `parser.rs` files with full parsing logic. When you regenerate, the code generator **skips existing files** and only creates new ones. This means you can safely regenerate to pick up new template features (like `visitor.rs`) without losing your parser implementation:

```
Skipping: .../src/lib.rs (already exists)
Skipping: .../src/arena.rs (already exists)
Skipping: .../src/parser.rs (already exists)
...
Outputting: .../src/visitor.rs
```

To force a full regeneration (e.g. into a fresh directory), use a new output path:

```bash
java -jar congocc.jar -lang rust -d /tmp/fresh-output YourGrammar.ccc
```

## AST Visitor

Generated parsers include a closure-based depth-first visitor via `Arena::visit()`:

```rust
use my_parser::*;
use std::any::Any;

let mut parser = Parser::new("your input".to_string())?;
let root = parser.parse()?;

// Count all nodes in the tree
let mut count = 0;
parser.arena().visit(root, &mut |_id, _node, _arena, depth, _opts| {
    count += 1;
    println!("Node at depth {}", depth);
    VisitControl::Continue
}, None);
```

The closure receives `(NodeId, &AstNode, &Arena, depth, Option<&dyn Any>)` and returns a `VisitControl`:

- **`Continue`** — visit children, then continue with siblings
- **`SkipChildren`** — skip this node's children, continue with siblings
- **`Stop`** — stop traversal entirely

The optional `options` parameter passes caller-supplied context (as `&dyn Any`) through to every closure invocation.

## Rust-Specific Notes

- **Naming conventions**: The generator automatically converts Java naming conventions to Rust's snake_case for methods/variables and UpperCamelCase for types
- **Documentation**: Generated code includes comprehensive doc comments
- **Lints**: The generated `lib.rs` includes recommended lint configuration
- **Edition**: Code is generated for Rust 2024 edition

### Acknowledgments

Anthropic's Claude (Claude Code 2.1.42, Opus 4.6) was used to generate most of the Rust code and documentation in this project. See [docs/command_prompts.md](docs/command_prompts.md) for prompt history.

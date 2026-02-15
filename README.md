# The Congo Parser Generator

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/congo-cc/congo-parser-generator)

The Congo Parser Generator is a Recursive Descent Parser Generator that generates code in Java, Python, C#, and Rust.

Here are some highlights:

## Grammars

Congo contains complete, up-to-date grammars for:

- [Java](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/java)
- [Python](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/python)
- [C#](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/csharp)
- [Lua](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/lua)
- [JSON](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/json)
- [Rust](https://github.com/congo-cc/congo-parser-generator/tree/master/examples/rust-test)

Any of these grammars may be freely used in your own projects, though it would be *nice* if you acknowledge the use and provide a link to this project. The above-linked grammars also can be studied as examples. (Best would be to start with the JSON grammar, move on to Lua, then Python, Java, and C# in order of complexity.)

## Cutting Edge Features

CongoCC has some features that, to the best of our knowledge are not in most (or possibly *any*) competing tools, such as:

- [Contextual Predicates](https://wiki.parsers.org/doku.php?id=contextual_predicates)
- [Context-sensitive tokenization](https://parsers.org/javacc21/activating-de-activating-tokens/)
- [Assertions](https://parsers.org/tips-and-tricks/introducing-assertions/) also [here](https://parsers.org/announcements/revisiting-assertions-introducing-the-ensure-keyword/)
- [Clean, streamlined up-to-here syntax](https://wiki.parsers.org/doku.php?id=up_to_here)
- [Support for the full 32-bit Unicode standard](https://parsers.org/javacc21/javacc-21-now-supports-full-unicode/)
- [Code Injection into generated classes](https://wiki.parsers.org/doku.php?id=code_injection_in_javacc_21)
- [Informative Stack Traces that include location information relative to the Grammar]()
- [A Preprocessor allowing one to turn on/off parts of the grammar based on various conditions](https://parsers.org/tips-and-tricks/javacc-21-has-a-preprocessor/)
- [An INCLUDE directive to allow large grammar files to be broken into multiple physical files](https://wiki.parsers.org/doku.php?id=include)

CongoCC also supports [fault-tolerant parsing](https://parsers.org/javacc21/the-promised-land-fault-tolerant-parsing/) that is admittedly in an unpolished, experimental stage, but basically usable.

If you are interested in this project, either as a user or developer, by all means sign up on our [Discussion Forum](https://discuss.congocc.org) and post any questions or suggestions there.

See our [QuickStart Guide](https://parsers.org/home/).

## Rust Code Generation

CongoCC can generate idiomatic Rust parsers with modern features including arena allocation, type-safe indices, and comprehensive error handling.

### Generating a Rust Parser

To generate a Rust parser from your grammar file:

```bash
java -jar congocc.jar -lang rust YourGrammar.ccc
```

The `-lang rust` option tells CongoCC to generate Rust code instead of the default Java.

### Specifying Output Directory

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

### Generated Files

CongoCC generates a complete Rust crate with the following structure:

- **`lib.rs`** - Module root with public API
- **`arena.rs`** - Arena allocator for AST nodes and tokens
- **`tokens.rs`** - Token type definitions and enums
- **`lexer.rs`** - Lexical analyzer (tokenizer)
- **`parser.rs`** - Recursive descent parser (scaffolding — see note below)
- **`error.rs`** - Error types with location tracking
- **`visitor.rs`** - Closure-based depth-first AST visitor
- **`Cargo.toml`** - Rust package manifest

### Compiling the Generated Parser

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

### Integrating into Your Application

#### Adding as a Dependency

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

#### Basic Usage

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

#### Working with AST Nodes (Arena-based)

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

#### Error Handling with Location Information

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

### Dependencies and Features

The generated Rust parser has **zero runtime dependencies** by default. However, you can enable optional features:

#### Optional Serde Support

To enable serialization of tokens and AST nodes, add to the generated `Cargo.toml`:

```toml
[dependencies.serde]
version = "1.0"
features = ["derive"]

[features]
default = []
serde = ["dep:serde"]
```

Then build with:
```bash
cargo build --features serde
```

#### Memory Characteristics

- **Arena allocation**: All AST nodes are stored in a contiguous memory arena
- **Type-safe indices**: `NodeId` and `TokenId` provide safe references without lifetimes
- **Zero-cost abstractions**: No `Rc<RefCell<>>` overhead
- **Cache-friendly**: Contiguous memory layout improves performance

#### Safety Guarantees

- **No unsafe code**: Generated parsers use only safe Rust
- **Result-based errors**: All parsing operations return `Result<T, ParseError>`
- **Compile-time checks**: Rust's type system catches errors at compile time

### Rust Examples

The `examples/rust-test/` directory contains two working Rust parser examples with comprehensive integration tests:

#### Arithmetic Parser

A simple arithmetic expression parser supporting `+`, `-`, `*`, `/`, parentheses, and integers.

```bash
# Run the existing tests (45 tests)
cd examples/rust-test/arithmetic
cargo test

# Regenerate from the grammar (preserves hand-written parser.rs)
java -jar congocc.jar -lang rust -d src SimpleArithmetic.ccc
cargo test
```

#### SQL Expression Parser

A SQL filter expression parser supporting boolean operators (`AND`, `OR`, `NOT`), comparisons (`=`, `<>`, `<`, `>`, `<=`, `>=`), `LIKE`, `IN`, `BETWEEN`, `IS NULL`/`IS NOT NULL`, arithmetic, and literals.

```bash
# Run the existing tests (65 tests, including visitor tests)
cd examples/rust-test/sqlexpr
cargo test

# Regenerate from the grammar (preserves hand-written parser.rs)
java -jar congocc.jar -lang rust -d src SqlExpr.ccc
cargo test
```

#### Regeneration Workflow

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

### AST Visitor

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

### Rust-Specific Notes

- **Naming conventions**: The generator automatically converts Java naming conventions to Rust's snake_case for methods/variables and UpperCamelCase for types
- **Documentation**: Generated code includes comprehensive doc comments
- **Lints**: The generated `lib.rs` includes recommended lint configuration
- **Edition**: Code is generated for Rust 2024 edition

#### Acknowledgments

Anthropic's Claude was used to generate most of the Rust code and documentation in this project. See [docs/command_prompts.md](docs/command_prompts.md) for prompt history.

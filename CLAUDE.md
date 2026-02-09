# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Congo Parser Generator (CongoCC)** is a recursive descent parser generator written in Java that generates parsers in Java, Python, C#, and Rust. It's a self-hosting compiler-compiler that uses its own bootstrap jar to rebuild itself.

## Build Commands

All build operations use Apache Ant. The project requires JDK 17+.

### Essential Commands

```bash
# Build the main jar (most common)
ant jar

# Clean all build artifacts
ant clean

# Run all tests (JSON, CICS, Preprocessor, Lua, C#, Java, Python parsers)
ant test

# Full bootstrap test: build with bootstrap jar, then rebuild with new jar
ant full-test

# Update the bootstrap jar in bin/ (run after significant changes)
ant update-bootstrap
```

### Individual Parser Tests

```bash
# Test specific parsers
ant test-java          # Java parser only
cd examples/json && ant test-all
cd examples/lua && ant test-all
cd examples/csharp && ant test-all
cd examples/python && ant test-all
```

### Running CongoCC

```bash
# Generate parser from a grammar file
java -jar congocc.jar [options] grammar-file.ccc

# Common options:
#   -d <dir>           Output directory
#   -lang <language>   Target language: java (default), python, csharp, rust
#   -jdk<N>            JDK target version (8-25, Java only)
#   -p key=value       Set preprocessor symbols
#   -q                 Quiet mode
#   -n                 Don't check for newer version

# Example: Generate Python parser
java -jar congocc.jar -lang python -d output/ grammar.ccc

# Example: Generate Rust parser
java -jar congocc.jar -lang rust -d output/ grammar.ccc
```

## Architecture

### Three-Phase Design

1. **Grammar Parsing**: CongoCC parses `.ccc` grammar files using its bootstrapped parser
2. **Semantic Analysis**: Builds internal Grammar representation and constructs NFA for lexer
3. **Code Generation**: Uses template engine (CTL) to generate language-specific parsers

### Key Package Structure

**`org.congocc.core`**
- Core data structures representing grammars, productions, and expansions
- `Grammar.java` - Root of grammar data structure
- `BNFProduction.java` - Grammar production rules
- `Expansion.java` - Abstract base for right-hand side elements
- `LexerData.java` - Lexical analysis structures
- `core/nfa/` - NFA construction for lexer generation

**`org.congocc.app`**
- `Main.java` - Entry point and command-line processing (src/java/org/congocc/app/Main.java:161)
- `AppSettings.java` - Configuration management with extensive boolean/string/integer settings
- `Errors.java` - Error reporting

**`org.congocc.codegen`**
- Multi-language code generation system
- `FilesGenerator.java` - Orchestrates file generation
- `TemplateGlobals.java` - Global template variables/functions
- `java/`, `python/`, `csharp/`, `rust/` - Language-specific generators
  - Each has: Formatter, Translator, Reaper (cleanup)
  - Java also has CodeInjector
  - Rust has RustTranslator (maps Java types to Rust, converts naming conventions)

**`org.congocc.templates`**
- Custom template engine (CTL - Congo Template Language)
- Similar to FreeMarker but tailored for code generation
- Templates in `src/templates/{java,python,csharp,rust}/*.ctl`

### Bootstrap Process

The project uses `bin/congocc.jar` (2.4MB) to rebuild itself:

1. Bootstrap jar generates parsers from grammars in `src/grammars/`
2. Generated Java code goes to `build/generated-java/`
3. Compilation combines `src/java/` + `build/generated-java/` → `build/`
4. New `congocc.jar` is created from compiled classes + templates
5. `ant full-test` validates by building twice (with old jar, then new jar)

### Internal Grammars

CongoCC uses 5 internal grammars (located in `src/grammars/`):

- `CongoCC.ccc` - Main CongoCC grammar specification
- `Lexical.inc.ccc` - Lexical specification (INCLUDE'd by CongoCC.ccc)
- `JavaInternal.ccc` - Java grammar (for parsing Java code in grammar files)
- `PythonInternal.ccc` - Python grammar (for parsing Python code snippets)
- `CSharpInternal.ccc` - C# grammar (for parsing C# code snippets)

These are rebuilt if modified via targets like `parser-gen`, `python-gen`, `csharp-gen`.

## Grammar File Format

Grammar files use `.ccc` extension with:

- **BNF Productions**: Define parsing rules
- **Lexical Specifications**: TOKEN/SKIP/UNPARSED definitions
- **Preprocessor Directives**: `#if`, `#define`, etc.
- **INCLUDE Directive**: Modular grammar composition
- **Code Injection**: Inject methods into generated classes
- **Assertions**: Grammar-level checks with ASSERT/ENSURE
- **Contextual Predicates**: Context-sensitive parsing decisions

### Built-in Grammar Includes

CongoCC provides aliases for including standard grammars (see AppSettings.java:64-79):

```
JAVA, JAVA_LEXER, JAVA_IDENTIFIER_DEF
PYTHON, PYTHON_LEXER, PYTHON_IDENTIFIER_DEF
CSHARP, CSHARP_LEXER, CSHARP_IDENTIFIER_DEF
JSON, JSONC
LUA
PREPROCESSOR
```

These map to `/include/{language}/*.ccc` files bundled in the jar.

## Examples Directory

The `examples/` directory contains production-quality grammars:

- **java/** - Complete Java grammar up to JDK 24, with JDK 8/11/17/21 variants
- **python/** - Full Python language support
- **csharp/** - Complete C# language support
- **lua/** - Lua 5.4.4, validated on 460K+ lines of WoW code
- **json/** - JSON and JSONC (with comments) parsers
- **preprocessor/** - C-style preprocessor
- **cics/** - CICS language
- **arithmetic/** - Simple arithmetic (good starting point)
- **rust-test/** - Rust examples with generated Rust parsers:
  - `arithmetic/` - Simple arithmetic expression parser
  - `sqlexpr/` - SQL expression parser (boolean logic, comparisons, LIKE, IN, BETWEEN, IS NULL)
  - Top-level standalone test executables (`test_arena.rs`, `test_lexer.rs`, `test_parser.rs`)

Start with JSON, then Lua, then Python/Java/C# in order of increasing complexity.

Each example has its own `build.xml` with targets like `clean`, `test`, `test-all`.

**Note:** Rust examples are NOT part of `ant test`. They must be built/tested separately with Cargo:

```bash
# Build and test Rust examples
cd examples/rust-test/arithmetic && cargo test
cd examples/rust-test/sqlexpr && cargo test
```

## Development Workflow

### Making Grammar Changes

1. Edit grammar files in `src/grammars/*.ccc`
2. Run `ant clean jar` to regenerate and rebuild
3. Test with `ant test` or specific example tests
4. For bootstrap jar updates: `ant update-bootstrap`

### Making Code Generator Changes

1. Edit Java sources in `src/java/org/congocc/`
2. Edit templates in `src/templates/{java,python,csharp,rust}/*.ctl`
3. Run `ant clean jar` to rebuild
4. Test generated code with `ant test`
5. For template changes affecting bootstrap: run `ant update-bootstrap` **twice** to ensure templates are fully updated

### Rust Code Generator Files

The Rust code generator consists of:
- `src/java/org/congocc/codegen/rust/RustTranslator.java` - Type/naming conversions
- `src/templates/rust/*.ctl` - Template files generating:
  - `lib.rs` - Module root with public API
  - `arena.rs` - Arena allocator for AST nodes (type-safe NodeId/TokenId)
  - `tokens.rs` - Token type definitions
  - `lexer.rs` - Lexical analyzer
  - `parser.rs` - Recursive descent parser
  - `error.rs` - Error types with location tracking
  - `Cargo.toml` - Package manifest (Rust 2024 edition)
- `src/templates/rust/tests/basic.rs.ctl` - Template for generating boilerplate integration tests
- Generated Rust parsers include `serde` and `serde_json` as dependencies

### Adding New Features

Features often span multiple layers:

1. **Grammar layer**: Update `src/grammars/CongoCC.ccc` to parse new syntax
2. **Core layer**: Add data structures in `org.congocc.core`
3. **Template layer**: Update code generation templates in `src/templates/`
4. **Codegen layer**: Modify language-specific translators/formatters if needed

### Testing Strategy

- **Unit tests**: Run `ant test` (tests all example parsers)
- **Bootstrap test**: Run `ant full-test` (critical for grammar/template changes)
- **Single parser test**: `cd examples/<name> && ant test` or `ant test-all`
- **Manual testing**: Generate parser with `java -jar congocc.jar` and test output

## Advanced Features

CongoCC includes several unique features:

- **Contextual Predicates**: Context-sensitive parsing decisions based on parse tree state
- **Context-sensitive Tokenization**: Tokens activated/deactivated based on parsing context
- **Up-to-here Syntax**: Clean syntax for certain parsing patterns
- **Full Unicode Support**: 32-bit Unicode standard
- **Code Injection**: Inject custom code into generated parser/lexer/token classes
- **Fault-tolerant Parsing**: Experimental error recovery (unpolished but usable)
- **Tree Building**: Automatic AST construction with JTB-style node generation
- **Token Chaining**: Token inheritance hierarchies

See README.md links for detailed documentation on these features.

## CI/CD

GitHub Actions workflow at `.github/workflows/core-tests.yml`:
- Runs on: Ubuntu, macOS, Windows
- Java versions: 17, 21
- Includes Jython and .NET SDK for cross-language testing

## Rust Development Notes

- Rust code generation is newer than Java/Python/C#; no Ant targets exist for Rust yet
- After modifying Rust templates, regenerate and test manually:
  ```bash
  ant clean jar
  java -jar congocc.jar -lang rust -d examples/rust-test/sqlexpr/src examples/rust-test/sqlexpr/SqlExpr.ccc
  cd examples/rust-test/sqlexpr && cargo test
  ```
- See `.claude/Learnings.md` for lessons learned on arena allocation, parser integration, and template backporting

## Debugging Tips

### Parser Generation Issues

- Check `build/generated-java/org/congocc/parser/` for generated parser code
- Add `-q` flag to suppress "checking for newer version" noise
- Use `-d` to control output directory for debugging

### Bootstrap Issues

If bootstrap jar is corrupted or incompatible:

```bash
ant restore-bootstrap-jar  # Restore from git
ant full-test              # Rebuild and test twice
```

### Template Debugging

Templates are in `src/templates/` and copied to `build/templates/` then bundled into jar.
If template changes don't take effect, ensure you're using the newly built jar, not the bootstrap jar.

## Important File Locations

- Bootstrap jar: `bin/congocc.jar`
- Build output: `congocc.jar` (root directory)
- Generated parsers: `build/generated-java/`
- Compiled classes: `build/org/`
- Templates (source): `src/templates/`
- Templates (bundled): Inside jar at `/templates/`
- Grammar includes (bundled): Inside jar at `/include/`

## License

MIT License with explicit clause: **Generated parser code is completely unencumbered and belongs solely to the user.**

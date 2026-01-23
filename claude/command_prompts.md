# Enhance CongoCC to Generate Rust Parsers

The CongoCC Parser Generator allows one to define a parser for a context-free language.  CongoCC is documented at https://parsers.org.  The Java source code for the parser generator is at https://github.com/congo-cc/congo-parser-generator.  The parsers can be generated in a number of different languages, most notably in the Java language, which is probably the best supported language.  The goal of this project is to enhance the CongoCC back-end code generator to produce parsers in the Rust language.  Please propose different approach to support the generation of parsers written in Rust.  Develop one or more plans for review, but don't implement any code yet.

## Please add information about rust support to the READMD.md file, including:
 

 1. How to invoke rust parser generation.
 2. How to specify where the generated rust parser source code is writtenj.
 3. How to compile the generated rust parser.
 4. How to integrate the generated rust parser into an application.
 5. Other usage information including code dependencies introduced by the parser. 
 
The generated Cargo.toml file assigns the Rust edition the value "2021".  Please have all generated Rust code use edition "2024"

The generated Cargo.toml file assigns "unknown" to the grammar_file field.  Please assign the absolute path of the .ccc grammar file passed in on the command line. 

## Implement Arena Allocation for Rust

Let's complete the of implementation of Rust code generation by fully supporting arena allocation.  Examine the project's Rust code and documentation to understand where we left off in the previous session.  The claude-plan.md file documents the previously developed plan.  Arena allocation support is not currently integrated into the parser.  Specifically, the example code under the "Working with AST Nodes (Arena-based)" heading in README.md does not appear to be correct:  The parser.arena() function is not defined.  Please analyze what implemention (if any) is needed to address the TODO in the RustTranslator.translateInvocation() method.  

Let's *reorganize*, *enhance* and *complete* the test code in examples/rust-test.  The reorganization should create two different example subdirectories, one for each of the .ccc grammar files that are currently in example/rust-test.  *SimpleArithmetic.ccc* should be moved to **examples/rust-test/arithmetic** along with all its generated code, integration tests, cargo files, etc. *SqlExprParser.ccc* should be moved to **examples/rust-test/sqlexpr**, which is where all its code and artifacts will be generated.

Both the arithmetic and sqlexpr parsers should be complete with leftover no TODOs in the code.  Currently, there are TODOs in methods in examples/rust-test/parser.rs and examples/rust-test/lexer.rs.  In addition, each of the examples should have integration tests that parse a number of input strings that comprehensively exercise the parser.  Positive tests parse strings that conform to each of the grammars, negative tests catch strings that do not conform to the grammars.  Currently, running "cargo test" in the arithmetic example causes a number of test failures.  All positive and negative tests should succeed.

Finally, both example parsers should include two types of integration tests.  The first test type simply parses each input string and asserts the expected outcome.  The second test type pretty prints the parsed AST to stdout.  

The ultimate goal is to complete the implementation of the rust parser generator with arena allocation support, to provide illustrative examples that validate the generated parsers, and to generate complete and up-to-date documentation. 

Please develop a plan to implement arena support and present the plan for review.  Also, record in a file all learnings in .claude/Learnings.md.
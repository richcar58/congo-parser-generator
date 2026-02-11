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

## Enhancing AST Capabilities

The generated Rust parsers can be use in two ways.  In README.md, the two usages patterns are shown under the *Basic Usage* and the *Working with AST Nodes (Arena-based)* headings.  The Basic Usage pattern is most appropriate for simply checking if a string is accepted by the parser.  The Arena-based pattern is used when the parsed input needs to be processed.  In this case, the caller obtains the root node of an AST, which can then be processed node by node using a depth first tree traversal.  As an example of AST traversal, each generated parser already comes with support for pretty printing the parsed input's AST.

Working with AST nodes can be made more straightforward by abstracting translations from NodeId or TokenId types.  Here are the requirements for enhanced Arena-based node traversal:

1. Each generated parser should save a copy of the original input string.  This string is publicly accessible including when the AST is being processed.  
2. Each AstNode should preserve and make easily accessible all information extracted during parsing.  In particular, the operators and operands of an expression node should be easily accessible.  For example, in the examples/rust-test/arithmetic directory, the *AdditiveExpressionNode* defined in arena.rs has *children*, *being_token* and *end_token* fields.  These fields have NodeId or TokenId types, which require translation to node types to extract the values present in the original input string.  In this case, it would be more convenient to supply *op()*, *left_operand()* and *right_operand()* methods for AST processing code to use.  Generally speaking, when processing code traverses an AST it should work with node types (e.g., MultiplicativeExpressionNode, ComparisonExpressionNode) rather than ID types (i.e., NodeId or TokenId).
3. The *pretty_print_impl()* method in the arithmetic and sqlexpr examples should be modified so that:
    1. The original input string is printed as part of the first line (i.e., the "AST:" line).
    2. Specific operator or operand values are always printed for nodes that have those values assigned.
    3. To reduce the output clutter, pass through nodes in the AST that have no assigned values except for a single child node should not be printed.

Allowing AST processing to work at the abstraction level of nodes rather than IDs should not impact performance.  Currently, the burden is on the processing code to translate ID types to node types.  This burden is simply shifted to generated parser code that AST processing code can call if it needs to.  Please develop a plan for these AST enhancements and present the plan for review.  Also, append all learnings to .claude/Learnings.md.

### Promulgating AST Enhancements

Please investigate how the AST enhancements just generated for rust-test/arithmetic and rust-test/sqlexpr can be back ported to the parser-generator source code that generates rust parsers.  The goal is that all the recent enhancements to the example code should be part of any parser generated in rust. Please develop a plan for back porting and present the plan for review.  Also, append all learnings to .claude/Learnings.md.

## Implement the Visitor Design Pattern

Analyze different approaches for generating the Visitor design pattern to operate on the node types defined in generated arena.rs files.  The basic requirement is for the parser-generator to create a visitor.rs file for each parser it generates.  This file defines the visitor object that takes a caller supplied function and applies it to each node visited during a depth-first traversal of the specified AST.  

Please research different approaches to implementing the Visitor design pattern in Rust and how they can be applied in this project.  Discuss the pros and cons of each approach and make recommendations.  Once we choose an approach, we'll create a plan.  Also, append all learnings to .claude/Learnings.md.

### Plan the Vistor Implementation 

Please create a plan for an Approach B: Closure-based Walker (Functional) implementation with the following changes:

**CHANGE 1**:  Change WalkControl enum name to VisitControl and change Arena's walk() method name to visit().

**CHANGE 2**:  Replace the current closure type signature on the visit() method: 

    FnMut(NodeId, &AstNode, &Arena) -> VisitControl

with an enhanced signature that allows for (1) a depth counter and (2) an optional parameter of any type:

    FnMut(NodeId, &AstNode, &Arena, usize, Option<&dyn Any) -> VisitControl

The fourth parameter is the **depth** of the node in the AST.  The root node has a depth of 0 and the visitor traversal code automatically increments the depth by 1 on each recursive call of the closure.   

The fifth optional parameter, **options**, is a reference to a dynamic type of the caller's choosing.  By supplying a reference, a closure can share state between its different invocations as well as with the calling code.

### Fix Parser Implementation

#### Problem Background 

There seems to have been a serious regression during some previous Claude session.  The generated parsers in examples/rust-test/arithmetic and examples/rust-test/sqlexpr do not fully implement their parser.rs methods when the parsers are regenerated.  Instead of actually generating code, the parse methods return the unit type and indicate with a TODO comment that the implementation needs to be completed.  Running "cargo test" causes integration_test.rs to fail in both examples. 

In the *Generated Files* section of README.md, a note indicates that parser.rs requires handwritten code in order to complete the parser.  This is not true.  The current example parser.rs files are complete and their code was Claude generated.

Another inaccuracy in README.md is in the *Optional Serde Support* section.  The Cargo.toml format shown is not valid, which may be involved in the parser.rs implementation problem.  The following format is the standard way of specifying serde dependancy:

    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

#### Fixing the Problem

There should never be a case where the parser requires handwritten code to be complete.  Here is how the example arithmetic parser is regenerated:

    cd examples/rust-test/arithmetic
    java -jar /home/rich/git/congo-rust/congocc.jar -d src -lang rust SimpleArithmetic.ccc

The example sqlexpr parser is regenerated in a similar way. 

The github repository at https://github.com/richcar58/congo-parser-generator.git contains the previously committed versions of the parser-generator.  Maybe the regression can be identified by looking at past commits.

The goal is to develop a plan for correctly generating and regenerating complete parsers all the time.  The generated parser.rs content should be the same as the current content.  Regeneration should output the same code and that code should pass all integration tests as they are currently written.  Append all learnings to .claude/Learnings.md.



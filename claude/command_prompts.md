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

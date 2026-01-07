use arithmetic::{Lexer, TokenType};

fn main() {
    // Test basic tokenization
    let test_input = "1 + 2 * (3 - 4) / 5".to_string();
    println!("Testing lexer with input: {}", test_input);
    println!();

    let mut lexer = Lexer::new(test_input);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("Token: {:?} = \"{}\" at {}..{}",
                    token.token_type,
                    token.image,
                    token.begin_offset,
                    token.end_offset
                );
                if token.token_type == TokenType::EOF {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("\nLexer test completed successfully!");
}

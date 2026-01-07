use arithmetic::Parser;

fn main() {
    let test_cases = vec![
        ("1", true),
        ("1 + 2", true),
        ("1 + 2 * 3", true),
        ("(1 + 2) * 3", true),
        ("1 + 2 * (3 - 4) / 5", true),
        ("((1 + 2) * (3 - 4)) / 5", true),
        ("1 +", false),  // Missing operand
        ("(1 + 2", false),  // Missing closing paren
        ("* 1", false),  // Invalid start
    ];

    println!("Testing parser with {} test cases\n", test_cases.len());

    let mut passed = 0;
    let mut failed = 0;

    for (input, should_pass) in test_cases {
        match Parser::new(input.to_string()) {
            Ok(mut parser) => {
                match parser.parse() {
                    Ok(_) => {
                        if should_pass {
                            println!("✓ PASS: \"{}\" parsed successfully", input);
                            passed += 1;
                        } else {
                            println!("✗ FAIL: \"{}\" should have failed but passed", input);
                            failed += 1;
                        }
                    }
                    Err(e) => {
                        if !should_pass {
                            println!("✓ PASS: \"{}\" correctly rejected: {}", input, e);
                            passed += 1;
                        } else {
                            println!("✗ FAIL: \"{}\" should have passed but failed: {}", input, e);
                            failed += 1;
                        }
                    }
                }
            }
            Err(e) => {
                if !should_pass {
                    println!("✓ PASS: \"{}\" correctly rejected during initialization: {}", input, e);
                    passed += 1;
                } else {
                    println!("✗ FAIL: \"{}\" failed to initialize: {}", input, e);
                    failed += 1;
                }
            }
        }
    }

    println!("\n{} passed, {} failed", passed, failed);

    if failed == 0 {
        println!("\nAll parser tests passed!");
    } else {
        std::process::exit(1);
    }
}

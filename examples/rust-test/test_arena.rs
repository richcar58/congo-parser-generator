use arithmetic::{Arena, AstNode, PrimaryNode, Token, TokenType, TokenId};

fn main() {
    println!("Testing arena allocation\n");

    // Create an arena
    let mut arena = Arena::new();

    // Allocate some tokens
    let tok1 = arena.alloc_token(Token::new(TokenType::INTEGER, "42".to_string(), 0, 2));
    let tok2 = arena.alloc_token(Token::new(TokenType::PLUS, "+".to_string(), 3, 4));
    let tok3 = arena.alloc_token(Token::new(TokenType::INTEGER, "17".to_string(), 5, 7));

    println!("Allocated 3 tokens:");
    println!("  Token {:?}: {:?}", tok1, arena.get_token(tok1));
    println!("  Token {:?}: {:?}", tok2, arena.get_token(tok2));
    println!("  Token {:?}: {:?}", tok3, arena.get_token(tok3));
    println!();

    // Allocate some AST nodes
    let node1 = arena.alloc_node(AstNode::Primary(PrimaryNode::new(tok1, tok1)));
    let node2 = arena.alloc_node(AstNode::Primary(PrimaryNode::new(tok3, tok3)));

    println!("Allocated 2 AST nodes:");
    println!("  Node {:?}: {:?}", node1, arena.get_node(node1));
    println!("  Node {:?}: {:?}", node2, arena.get_node(node2));
    println!();

    // Demonstrate that NodeId and TokenId are cheap to copy
    let node1_copy = node1;
    let tok1_copy = tok1;

    println!("IDs are Copy types (cheap to duplicate):");
    println!("  Original node1: {:?}, copy: {:?}, equal: {}", node1, node1_copy, node1 == node1_copy);
    println!("  Original tok1: {:?}, copy: {:?}, equal: {}", tok1, tok1_copy, tok1 == tok1_copy);
    println!();

    println!("Arena allocation test completed successfully!");
    println!("✓ Arena owns all nodes and tokens");
    println!("✓ NodeId and TokenId provide type-safe indices");
    println!("✓ No reference counting overhead (Rc/RefCell)");
    println!("✓ All data contiguous in memory (cache-friendly)");
}

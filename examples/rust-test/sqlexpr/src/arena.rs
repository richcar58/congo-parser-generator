//! Arena allocator for SQL expression AST nodes

use crate::tokens::Token;

/// Type-safe index for nodes in the arena
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Type-safe index for tokens in the arena
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TokenId(pub usize);

/// Arena that owns all AST nodes and tokens
pub struct Arena {
    /// All AST nodes
    nodes: Vec<AstNode>,
    /// All tokens
    tokens: Vec<Token>,
}

impl Arena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Arena {
            nodes: Vec::new(),
            tokens: Vec::new(),
        }
    }

    /// Allocate a new node in the arena
    pub fn alloc_node(&mut self, node: AstNode) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(node);
        id
    }

    /// Get a reference to a node
    pub fn get_node(&self, id: NodeId) -> &AstNode {
        &self.nodes[id.0]
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: NodeId) -> &mut AstNode {
        &mut self.nodes[id.0]
    }

    /// Allocate a new token in the arena
    pub fn alloc_token(&mut self, token: Token) -> TokenId {
        let id = TokenId(self.tokens.len());
        self.tokens.push(token);
        id
    }

    /// Get a reference to a token
    pub fn get_token(&self, id: TokenId) -> &Token {
        &self.tokens[id.0]
    }

    /// Pretty print the AST starting from the given node, with the original input
    pub fn pretty_print(&self, root: NodeId, indent: usize, input: &str) -> String {
        let mut result = format!("AST: \"{}\"\n", input);
        self.pretty_print_impl(root, indent + 1, &mut result);
        result
    }

    /// Check if a node is a pass-through (single child, no semantic value)
    fn is_passthrough(&self, node_id: NodeId) -> bool {
        match self.get_node(node_id) {
            AstNode::SqlExpression(n) => n.children.len() == 1,
            AstNode::OrExpression(n) => n.children.len() == 1,
            AstNode::AndExpression(n) => n.children.len() == 1,
            AstNode::NotExpression(n) => n.children.len() == 1 && !n.is_negated,
            AstNode::ComparisonExpression(n) => n.children.len() == 1 && n.comparison_op.is_none(),
            AstNode::ValueList(_) => false,
            AstNode::AdditiveExpression(n) => n.children.len() == 1 && n.operators.is_empty(),
            AstNode::MultiplicativeExpression(n) => n.children.len() == 1 && n.operators.is_empty(),
            AstNode::UnaryExpression(n) => n.children.len() == 1 && !n.is_negated,
            AstNode::PrimaryExpression(_) => false,
        }
    }

    /// Get the first child of a node (if any)
    fn first_child(&self, node_id: NodeId) -> Option<NodeId> {
        match self.get_node(node_id) {
            AstNode::SqlExpression(n) => n.children.first().copied(),
            AstNode::OrExpression(n) => n.children.first().copied(),
            AstNode::AndExpression(n) => n.children.first().copied(),
            AstNode::NotExpression(n) => n.children.first().copied(),
            AstNode::ComparisonExpression(n) => n.children.first().copied(),
            AstNode::ValueList(n) => n.children.first().copied(),
            AstNode::AdditiveExpression(n) => n.children.first().copied(),
            AstNode::MultiplicativeExpression(n) => n.children.first().copied(),
            AstNode::UnaryExpression(n) => n.children.first().copied(),
            AstNode::PrimaryExpression(n) => n.children.first().copied(),
        }
    }

    fn pretty_print_impl(&self, node_id: NodeId, indent: usize, result: &mut String) {
        // Skip pass-through nodes
        if self.is_passthrough(node_id) {
            if let Some(child) = self.first_child(node_id) {
                self.pretty_print_impl(child, indent, result);
            }
            return;
        }

        let indent_str = "  ".repeat(indent);
        match self.get_node(node_id) {
            AstNode::SqlExpression(node) => {
                result.push_str(&format!("{}SqlExpression\n", indent_str));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::OrExpression(node) => {
                if node.children.len() > 1 {
                    result.push_str(&format!("{}OrExpression [OR x{}]\n", indent_str, node.children.len() - 1));
                } else {
                    result.push_str(&format!("{}OrExpression\n", indent_str));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::AndExpression(node) => {
                if node.children.len() > 1 {
                    result.push_str(&format!("{}AndExpression [AND x{}]\n", indent_str, node.children.len() - 1));
                } else {
                    result.push_str(&format!("{}AndExpression\n", indent_str));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::NotExpression(node) => {
                if node.is_negated {
                    result.push_str(&format!("{}NotExpression [NOT]\n", indent_str));
                } else {
                    result.push_str(&format!("{}NotExpression\n", indent_str));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::ComparisonExpression(node) => {
                if let Some(op) = &node.comparison_op {
                    let op_str = match op {
                        ComparisonOp::Eq => "=",
                        ComparisonOp::Ne => "<>",
                        ComparisonOp::Lt => "<",
                        ComparisonOp::Gt => ">",
                        ComparisonOp::Le => "<=",
                        ComparisonOp::Ge => ">=",
                        ComparisonOp::Like => "LIKE",
                        ComparisonOp::In => "IN",
                        ComparisonOp::Between => "BETWEEN",
                        ComparisonOp::IsNull => "IS NULL",
                        ComparisonOp::IsNotNull => "IS NOT NULL",
                    };
                    result.push_str(&format!("{}ComparisonExpression [{}]\n", indent_str, op_str));
                } else {
                    result.push_str(&format!("{}ComparisonExpression\n", indent_str));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::ValueList(node) => {
                result.push_str(&format!("{}ValueList [{} items]\n", indent_str, node.children.len()));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::AdditiveExpression(node) => {
                let ops: Vec<&str> = node.operators.iter()
                    .map(|op| match op {
                        AdditiveOp::Add => "+",
                        AdditiveOp::Sub => "-",
                    })
                    .collect();
                if ops.is_empty() {
                    result.push_str(&format!("{}AdditiveExpression\n", indent_str));
                } else {
                    result.push_str(&format!("{}AdditiveExpression [{}]\n", indent_str, ops.join(", ")));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::MultiplicativeExpression(node) => {
                let ops: Vec<&str> = node.operators.iter()
                    .map(|op| match op {
                        MultiplicativeOp::Mul => "*",
                        MultiplicativeOp::Div => "/",
                    })
                    .collect();
                if ops.is_empty() {
                    result.push_str(&format!("{}MultiplicativeExpression\n", indent_str));
                } else {
                    result.push_str(&format!("{}MultiplicativeExpression [{}]\n", indent_str, ops.join(", ")));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::UnaryExpression(node) => {
                if node.is_negated {
                    result.push_str(&format!("{}UnaryExpression [-]\n", indent_str));
                } else {
                    result.push_str(&format!("{}UnaryExpression\n", indent_str));
                }
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
            AstNode::PrimaryExpression(node) => {
                let token = self.get_token(node.begin_token);
                result.push_str(&format!("{}PrimaryExpression(\"{}\")\n", indent_str, token.image));
                for child in &node.children {
                    self.pretty_print_impl(*child, indent + 1, result);
                }
            }
        }
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

/// Enum containing all AST node types
#[derive(Debug, Clone)]
pub enum AstNode {
    /// Root SQL expression node
    SqlExpression(SqlExpressionNode),
    /// OR boolean expression (e.g., `a OR b`)
    OrExpression(OrExpressionNode),
    /// AND boolean expression (e.g., `a AND b`)
    AndExpression(AndExpressionNode),
    /// NOT boolean expression (e.g., `NOT a`)
    NotExpression(NotExpressionNode),
    /// Comparison expression (e.g., `a = b`, `x > 5`, `name LIKE '%test%'`)
    ComparisonExpression(ComparisonExpressionNode),
    /// List of values for IN expressions (e.g., `(1, 2, 3)`)
    ValueList(ValueListNode),
    /// Additive expression with + or - operators
    AdditiveExpression(AdditiveExpressionNode),
    /// Multiplicative expression with * or / operators
    MultiplicativeExpression(MultiplicativeExpressionNode),
    /// Unary expression with optional negation
    UnaryExpression(UnaryExpressionNode),
    /// Primary expression: literal, identifier, or parenthesized expression
    PrimaryExpression(PrimaryExpressionNode),
}

/// AST node for SqlExpression production (root of the parse tree)
#[derive(Debug, Clone)]
pub struct SqlExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl SqlExpressionNode {
    /// Create a new SqlExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        SqlExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for OrExpression production (handles OR boolean operator)
#[derive(Debug, Clone)]
pub struct OrExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (operands joined by OR)
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl OrExpressionNode {
    /// Create a new OrExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        OrExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for AndExpression production (handles AND boolean operator)
#[derive(Debug, Clone)]
pub struct AndExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (operands joined by AND)
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl AndExpressionNode {
    /// Create a new AndExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        AndExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// AST node for NotExpression production (handles NOT boolean operator)
#[derive(Debug, Clone)]
pub struct NotExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
    /// Whether the NOT operator was present
    pub is_negated: bool,
}

impl NotExpressionNode {
    /// Create a new NotExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        NotExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            is_negated: false,
        }
    }
}

/// AST node for ComparisonExpression production (handles comparison operators)
#[derive(Debug, Clone)]
pub struct ComparisonExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (left and right operands)
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
    /// The comparison operator used, if any
    pub comparison_op: Option<ComparisonOp>,
}

/// Comparison operation type for SQL expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    /// Equality (=)
    Eq,
    /// Not equal (<> or !=)
    Ne,
    /// Less than (<)
    Lt,
    /// Greater than (>)
    Gt,
    /// Less than or equal (<=)
    Le,
    /// Greater than or equal (>=)
    Ge,
    /// Pattern matching (LIKE)
    Like,
    /// Set membership (IN)
    In,
    /// Range check (BETWEEN)
    Between,
    /// Null check (IS NULL)
    IsNull,
    /// Not null check (IS NOT NULL)
    IsNotNull,
}

impl ComparisonExpressionNode {
    /// Create a new ComparisonExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        ComparisonExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            comparison_op: None,
        }
    }
}

/// AST node for ValueList production (list of values for IN expressions)
#[derive(Debug, Clone)]
pub struct ValueListNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (the values in the list)
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl ValueListNode {
    /// Create a new ValueListNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        ValueListNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }
}

/// Operator for additive expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdditiveOp {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Sub,
}

/// AST node for AdditiveExpression production (handles + and - operators)
#[derive(Debug, Clone)]
pub struct AdditiveExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (operands)
    pub children: Vec<NodeId>,
    /// Operators between children: operators[i] is between children[i] and children[i+1]
    pub operators: Vec<AdditiveOp>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl AdditiveExpressionNode {
    /// Create a new AdditiveExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        AdditiveExpressionNode {
            parent: None,
            children: Vec::new(),
            operators: Vec::new(),
            begin_token,
            end_token,
        }
    }

    /// Get the left operand (first child)
    pub fn left<'a>(&self, arena: &'a Arena) -> &'a AstNode {
        arena.get_node(self.children[0])
    }

    /// Get the right operand (second child for binary case)
    pub fn right<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.get(1).map(|id| arena.get_node(*id))
    }

    /// Get operator at index (between children[i] and children[i+1])
    pub fn op(&self, index: usize) -> Option<AdditiveOp> {
        self.operators.get(index).copied()
    }

    /// Get first operator (for binary expressions)
    pub fn first_op(&self) -> Option<AdditiveOp> {
        self.operators.first().copied()
    }
}

/// Operator for multiplicative expressions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiplicativeOp {
    /// Multiplication (*)
    Mul,
    /// Division (/)
    Div,
}

/// AST node for MultiplicativeExpression production (handles * and / operators)
#[derive(Debug, Clone)]
pub struct MultiplicativeExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (operands)
    pub children: Vec<NodeId>,
    /// Operators between children: operators[i] is between children[i] and children[i+1]
    pub operators: Vec<MultiplicativeOp>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl MultiplicativeExpressionNode {
    /// Create a new MultiplicativeExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        MultiplicativeExpressionNode {
            parent: None,
            children: Vec::new(),
            operators: Vec::new(),
            begin_token,
            end_token,
        }
    }

    /// Get the left operand (first child)
    pub fn left<'a>(&self, arena: &'a Arena) -> &'a AstNode {
        arena.get_node(self.children[0])
    }

    /// Get the right operand (second child for binary case)
    pub fn right<'a>(&self, arena: &'a Arena) -> Option<&'a AstNode> {
        self.children.get(1).map(|id| arena.get_node(*id))
    }

    /// Get operator at index (between children[i] and children[i+1])
    pub fn op(&self, index: usize) -> Option<MultiplicativeOp> {
        self.operators.get(index).copied()
    }

    /// Get first operator (for binary expressions)
    pub fn first_op(&self) -> Option<MultiplicativeOp> {
        self.operators.first().copied()
    }
}

/// AST node for UnaryExpression production (handles unary minus)
#[derive(Debug, Clone)]
pub struct UnaryExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
    /// Whether the unary minus operator was present
    pub is_negated: bool,
}

impl UnaryExpressionNode {
    /// Create a new UnaryExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        UnaryExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
            is_negated: false,
        }
    }
}

/// AST node for PrimaryExpression production (literals, identifiers, parenthesized expressions)
#[derive(Debug, Clone)]
pub struct PrimaryExpressionNode {
    /// Parent node in the AST, if any
    pub parent: Option<NodeId>,
    /// Child nodes (for parenthesized expressions)
    pub children: Vec<NodeId>,
    /// First token of this node's span
    pub begin_token: TokenId,
    /// Last token of this node's span
    pub end_token: TokenId,
}

impl PrimaryExpressionNode {
    /// Create a new PrimaryExpressionNode with the given token span
    pub fn new(begin_token: TokenId, end_token: TokenId) -> Self {
        PrimaryExpressionNode {
            parent: None,
            children: Vec::new(),
            begin_token,
            end_token,
        }
    }

    /// Get the token image (the actual value as string)
    pub fn value<'a>(&self, arena: &'a Arena) -> &'a str {
        &arena.get_token(self.begin_token).image
    }
}

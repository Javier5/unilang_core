#[derive(Debug, Clone)]
pub enum ASTNode<'a> {
    IfElse {
        condition: Box<ASTNode<'a>>,
        body_true: Vec<ASTNode<'a>>,
        body_false: Option<Vec<ASTNode<'a>>>,
    },
    Expression(String),
}

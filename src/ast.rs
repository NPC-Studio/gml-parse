/// All of the syntax nodes which we can produce. This is "abstract"
/// because it cannot be used to fully recreate the source material (it lacks
/// columns and lines and comments and whitespace).
#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
pub enum AbstractSyntaxNode {
    /// An enum has some enumerated list of children.
    Enum(Enum),

    /// Some arbitrary Identifier
    Identifier(Identifier),
}

impl AbstractSyntaxNode {
    /// Unwraps this node as an identifier.
    ///
    /// ## Panics
    /// If the AST Node is not an Identifier, panics.
    pub fn unwrap_identifier(self) -> Identifier {
        if let Self::Identifier(i) = self {
            i
        } else {
            panic!("tried to unwrap ast as identifier, but it was not")
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Clone)]
pub struct Enum {
    pub name: Identifier,
    pub variants: Vec<Identifier>,
}

impl Enum {
    pub fn new(name: Identifier, variants: Vec<Identifier>) -> Self {
        Self { name, variants }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Identifier(pub &'static str);
impl Identifier {
    pub fn new(inner: &'static str) -> Self {
        Self(inner)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    FUNCTION,
    METHOD,
    INITIALIZER,
    NONE,
    LAMBDA,
}


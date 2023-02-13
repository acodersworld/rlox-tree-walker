#[derive(Debug)]
pub enum EvalValue {
    Number(f32),
    Str(String),
    Bool(bool),
    Nil,
}

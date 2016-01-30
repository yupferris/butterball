#[derive(Debug)]
pub struct Program {
    pub function_table: Vec<Function>
}

#[derive(Debug)]
pub struct Function {
    pub name: String
}

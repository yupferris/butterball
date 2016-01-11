#[derive(Debug)]
pub enum Node {
    Comment(String)
}

#[derive(Debug)]
pub struct Root {
    pub nodes: Vec<Node>
}

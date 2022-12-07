
pub type Stack = Vec<char>;


#[derive(Debug)]
pub struct Instruction {
    pub amount: usize,
    pub from: usize,
    pub to: usize
}
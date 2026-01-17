struct Stack<T> {
    data: Vec<T>,
}

impl<T: std::fmt::Debug> Stack<T> {
    fn new() -> Self {
        Stack {
            data: Vec::new(),
        }
    }
    fn push(&mut self, val: T){
        self.data.push(val);     
    }
    fn pop(&mut self) -> Option<T>{
        if self.data.is_empty(){
            return None;
        }
        self.data.pop()
    }
    fn peek(&self) -> Option<&T>{
        self.data.last()
    }
    fn print_all(&self){
        for i in &self.data{
            println!("{:?}" , i);
        }
    }
}







fn main() {
    let mut stack: Stack<i64> = Stack::<i64>::new();
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.push(1);
    stack.pop();
    stack.peek();
    stack.print_all();
}


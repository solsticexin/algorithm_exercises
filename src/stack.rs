#[derive(Debug)]
pub struct Stack<T>{
    size:usize,
    data:Vec<T>
}
impl<T> Stack<T>  {
    pub fn new()->Self{
        Self{
            size:0,
            data:Vec::new()
        }
    }
    pub fn is_empty(&self)->bool{
        self.size==0
    }
    pub fn len(&self)->usize{
        self.size
    }
    pub fn clear(&mut self){
        self.size=0;
        self.data.clear();
    }
    pub fn push(&mut self,val:T){
        self.data.push(val);
        self.size+=1;
    }
    pub fn pop(&mut self)->Option<T>{
        if self.size==0 { return None }
        let del=self.data.pop();
        self.size-=1;
        del
    }
}
pub fn par_checker(par:&str){
    let mut char_list=Vec::new();
    for char in par.chars() {
        char_list.push(char);
    }
    let mut index=0;
    let mut balance=true;
    let mut stack:Stack<char>=Stack::new();
}
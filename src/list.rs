pub mod sq_list;
const   MAX_SIZE: usize=50;
pub struct SqList<T>{
    data:[T;MAX_SIZE],
    len:usize,
}

impl<T> SqList<T>
where T:PartialOrd+Copy+Default
{
    pub fn new()->Self{
        Self { data: [T::default();MAX_SIZE], len:0 }
    }

    pub fn insert(&mut self,i:usize,e:T)->bool{
        if  i==0 || i>self.len+1{
            return false;
        }
        if self.len>=MAX_SIZE {
            return false;
        }
        for j in (i..=self.len).rev() {
            self.data[j]=self.data[j-1];
        }
        self.data[i-1]=e;
        self.len=self.len+1;
        true
    }

    pub fn delete(&mut self,i:usize)->Option<T>{
        if i==0 || i>self.len {
            return None;
        }
        let result=Some(self.data[i-1]);
        for j in i..self.len {
            self.data[j-1]=self.data[j];
        }
        self.len=self.len-1;
        result
    }
    pub fn locate_elem(&self,elem:T)->usize{
        for i in 0..=self.len{
            if self.data[i] ==elem {
                return i+1;
            }
        }
        0
    }
    
}
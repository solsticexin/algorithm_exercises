use std::fmt::Debug;

///顺序表的最大容量
const MAX_SIZE:usize=50;
///静态顺序表结构
struct SqList<ElemType>{
    data:[ElemType;MAX_SIZE],
    len:usize,
}

impl<ElemType> SqList<ElemType>
where ElemType:Copy +Default+Debug+PartialEq
{
    pub fn new()->SqList<ElemType>{
        Self{
            data:[ElemType::default();MAX_SIZE],
            len:0,
        }
    }
    pub fn len(&self)->usize{
        self.len
    }
    pub fn is_empty(&self)->bool{
        self.len == 0
    }
    ///采用位序 按值查找 返回0则代表没有找到
    pub fn locate(&self,e:ElemType)->usize{
        for (i,data) in self.data.iter().enumerate() {
            if *data == e{
                return i;
            }
        }
        0
    }
    pub fn get(&self,i:usize)->Option<ElemType>{
        if i>self.len && i > MAX_SIZE && i == 0{
            return None;
        }
        Some(self.data[i])
    }
    pub fn insert(&mut self,i:usize,e:ElemType)->bool{
        if i>self.len+1 && i ==0 {
            return false;
        }
        for i in (i..=self.len).rev() {
            self.data[i]=self.data[i-1];
        }
        self.data[i-1]=e;
        true
    }
}

//动态分配的顺序表 --不是链表
struct SeqList<ElemType>{
    data:Vec<ElemType>,
}
impl <ElemType> SeqList<ElemType>
where ElemType:Clone+Collection
{
    pub fn new()->SeqList<ElemType> {
        SeqList{
            data:Vec::with_capacity(MAX_SIZE)
        }
}

}
pub trait Collection{
    type Item;
    fn len(&self)->usize;
    //按值查找
    fn locate(&self,e:Self::Item)->usize;
    ///按位查找 返回option
    fn get(&self,i:usize)->Option<Self::Item>;
    fn insert(&mut self,i:usize,e:Self::Item)->bool;
    ///删除元素 返回option
    fn delete(&mut self,i:usize)->Option<Self::Item>;
    fn print(&self);
    fn is_empty(&self)->bool;
}
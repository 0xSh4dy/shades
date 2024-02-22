pub trait RegOperations{
    fn ro_add(&mut self,reg1:usize,reg2:usize)->i32;
    fn ro_sub(&mut self,reg1:usize,reg2:usize)->i32;
    fn ro_mul(&mut self,reg1:usize,reg2:usize)->i32;
    fn ro_div(&mut self,reg1:usize,reg2:usize)->i32;
    fn ro_load(&mut self,val:i64)->i32;
}
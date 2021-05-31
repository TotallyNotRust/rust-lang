pub struct Math {}
impl Math {
    pub fn multiply(x:i32,y:i32) -> Result<i32, ()>{
        if x==0 || y==0 {
            return Err(())
        }
        let output = x*y;
        return Ok(output)
    }
    pub fn divide(x:u8,y:u8) -> Result<u8, ()> {
        let output: u8 = x/y;
        return Ok(output)
    }
}
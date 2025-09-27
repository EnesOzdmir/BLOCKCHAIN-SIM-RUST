use sha256::digest;
#[derive(Debug)]
pub struct Block {
    pub(crate)index:u32,
    pub(crate)hash:String,
    pub(crate)prevhash:String,
    pub(crate)nonce:u32,
    pub(crate)data:String,
}





impl Block {
    pub fn block_create(index:u32,data:String) -> Block {
        Block {
            index,
            data,
            nonce:0,
            prevhash:"".to_string(),
            hash:"".to_string(),
        }
    }

    pub fn block_update(&mut self,current_nonce:u32) -> Self {
        let a = format!("Enes {} king babuş {} kraliyo selamlar,{} babaya babu {} {}",self.prevhash,self.index,self.nonce, current_nonce,self.data);
        let new_hash= digest(a.as_bytes());
        Self{
            index:self.index,
            data:self.data.trim().to_string(),
            nonce:current_nonce,
            prevhash:self.hash.clone(),
            hash:new_hash,
        }
    }
    
    
    pub fn mine(&mut self) -> Self {
        let mut  i =0;
        let mut a :String = String::from("111111111111111111") ;
        let mut z :String = String::new() ;
        while "000" != &a[0..3] {
               a = self.block_update(i).hash;
            i +=1;
        }

        Self{
            index:self.index,
            data:self.data.trim().to_string(),
            nonce:i,
            prevhash:self.hash.clone(),
            hash:a,
        }
    }


}
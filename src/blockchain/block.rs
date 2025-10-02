use std::sync::atomic::{AtomicU32, Ordering};
use sha256::digest;
use std::sync::Mutex;
use lazy_static::lazy_static;
use mini_redis::client;
use mini_redis::client::Client;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub struct Block {
    pub(crate)index:u32,
    pub(crate)hash:String,
    pub(crate)prevhash:String,
    pub(crate)nonce:u32,
    pub(crate)data:String,
}


static lorem: &str = "lorem";

impl Block {

    pub fn block_create(data:String) -> Block {


            Block {
                index: 1,
                data,
                nonce: 0,
                prevhash: "Neden".to_string(),
                hash: "".to_string(),
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
    
    
    pub  fn mine(&mut self, prevhash_value:String) -> Self {

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
            prevhash:prevhash_value,
            hash:a,
        }
    }



    pub async fn connect<T: ToSocketAddrs>(addr: T) -> mini_redis::Result<Client> {
        client::connect(addr).await
    }
}
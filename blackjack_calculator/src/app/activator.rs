use std::io::{Read, Write};
use super::*;



pub struct Activator {
    code: Option<String>,
    pub activated:bool,
}
impl Activator {
    pub fn new() -> Activator{
        let mut _self = Activator {
            code:match Self::load_code(){
                Ok(x) => Some(x),
                Err(_) => None,
            },
            activated:false,
        };
        _self.check_activated();
        _self
    }
    pub fn check_activated(&mut self) -> bool{
        let temp = self.code.is_some() && code_gen_lib::check_code(self.code.as_ref().unwrap());
        self.activated = temp;
        temp
    }
    pub fn unactivate(&mut self){
        self.code = None;
        self.activated = false;
    }
    pub fn activate(&mut self,code:&str) -> Result<(),String>{
        self.code = Some(match code.parse(){
            Ok(o) => o,
            Err(_) => return Err("Invalid string in the activation code.".to_string()),
        });
        self.check_activated();
        if self.activated{
            let mut file = match std::fs::File::create(ACTIVATION_CODE_PATH){  
                Ok(o) => o,
                Err(_) => return Err("Failed to save activation code.".to_string()),
            };
            match write!(file,"{}",self.code.as_ref().unwrap()){  
                Ok(o) => o,
                Err(_) => return Err("Failed to save activation code!".to_string()),
            };
        }else{
            return Err("Invalid activation code.".to_string());
        }
        Ok(())
    }
    fn load_code() -> Result<String,()>{
        let mut file = match std::fs::File::open(ACTIVATION_CODE_PATH){
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        Ok(string)
    }
}
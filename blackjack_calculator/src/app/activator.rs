use std::{io::{Read, Write}, os::windows::process::CommandExt};
use super::*;



pub struct Activator {
    code: Option<u64>,
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
        let temp = self.code.is_some() && Self::get_hash() == self.code.unwrap();
        self.activated = temp;
        temp
    }
    pub fn unactivate(&mut self){
        self.code = None;
        self.activated = false;
    }
    pub fn activate(&mut self,code:&String) -> Result<(),String>{
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
            match write!(file,"{:?}",self.code.unwrap()){  
                Ok(o) => o,
                Err(_) => return Err("Failed to save activation code.".to_string()),
            };
        }else{
            return Err("Invalid activation code.".to_string());
        }
        Ok(())
    }
    fn load_code() -> Result<u64,()>{
        let mut file = match std::fs::File::open(ACTIVATION_CODE_PATH){
            Ok(x) => x,
            Err(_) => return Err(()),
        };
        let mut string = String::new();
        file.read_to_string(&mut string).unwrap();
        match string.parse(){
            Ok(x) => Ok(x),
            Err(_) => Err(()),
        }
    }
    fn get_hash() -> u64{
        let temp = Activator::get_pcid();
        code_gen_lib::generate_hash(temp)
    }
    pub fn get_pcid() -> String {
        let process = std::process::Command::new("reg",)
        .args(&["query","HKEY_LOCAL_MACHINE\\SOFTWARE\\Microsoft\\Cryptography","/v","MachineGuid"])
        .stdout(std::process::Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .unwrap();
        let mut string = String::new();
        process.stdout.unwrap().read_to_string(&mut string).unwrap();
        string.split(" ").last().unwrap().trim().to_owned()
    }
}

#[cfg(test)]
mod test{
    use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

    use super::*;
    #[test]
    fn test(){
        let temp = Activator::get_pcid();
        let mut hasher = DefaultHasher::new();
        temp.hash(&mut hasher);
        println!("{}",hasher.finish());
    }
}
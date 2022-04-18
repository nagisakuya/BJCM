use std::{fs::File, io::Write};


//work on progress
struct Dout{
    
}

fn double_output(st:String,file:&mut File){
    print!("{}",st);
    file.write_all(st.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn func(range:impl Iterator<Item = usize>){
    for _ in range{
        todo!();
    }
}

fn main(){
    func(0..10);
    func(0..=9);
}
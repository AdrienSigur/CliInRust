use std::env;



fn main(){

    let input : Vec<String> = env::args().collect();

    if input.len() != 2 {
        println!("You have only {} argument you need more" , input.len());
    }else{
        println!("{}", input.len());
    }

}
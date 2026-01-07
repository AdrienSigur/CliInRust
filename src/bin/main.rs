
mod structure;
use structure::User;


fn main() {
    println!("Hello, world!");


    let mut  x : i32  ;
    x = 10 ; 
    x += 15 ;

    let typecalcul  : &str = "Sousttion";

    let y : i32 = 5 ;

    println!("Le résultat de y est de : {} et celui de x : {}", y , x);

    calcul(x , y , typecalcul);


    let mut user2  = User::build_user("eidowardo@gmail.com".to_string(),"eidowardo".to_string());

    user2.userprint();

    

   
}




fn calcul(  x : i32 , y : i32 , calcul : &str) {
    

    let result  = match calcul {
        "Addition" => x + y ,
        "Soustraction" => x - y ,
        "Division" => x / y ,
        "Multiplication" => x * y  ,
        _ => 0 ,

    };

    if result ==  0{
        println!("This operation {} is not available for this feature" , calcul )
    }else{
            println!("The result of this operation {} is {}" , calcul , result)
    } 

    
}


   




// fn divide(x : i32 , y : i32){

//     let result : i32 = x ;

// }

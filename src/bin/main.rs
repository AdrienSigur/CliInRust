

fn main() {
    println!("Hello, world!");


    let mut  x : i32  ;
    x = 10 ; 
    x += 15 ;

    let typecalcul  : &str = "Sousttion";

    let y : i32 = 5 ;

     println!("Le résultat de y est de : {} et celui de x : {}", y , x);

    calcul(x , y , typecalcul);


    // Get an RNG:
// let mut rng = rand::rng();



// Generate and shuffle a sequence:
// let mut nums: Vec<i32> = (1..4).collect();

// nums.shuffle(&mut rng);

// let value = nums.choose(&mut rng);

// match value.copied() {
//         Some(1) => println!("skibidi"),
//         Some(2) => println!("bitch"),
//         Some(3) => println!("jojo"),
        
//         // ⚠️ OBLIGATOIRE : Rust t'oblige à gérer le cas "Et si c'est vide ?"
//         None => println!("La liste est vide !"),
        
//         // Le cas "Tout le reste" (si jamais ta liste avait un 4, 5...)
//         _ => println!("Un autre nombre"),
// }



   
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
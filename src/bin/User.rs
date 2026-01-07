use std::{env};
use std::path::Path;

fn main(){
    struct User <'a> {
        nom : &'a str,
        prenom : &'a str
    }


    impl User<'_>  {
        fn nameoutput(&self){
            println!("Bonjour je m'appelle {} et mon nom est {}" , self.prenom , self.nom);

        }
    }


    let user =  User { nom : "sigur" , prenom : "adrien" };

    User::nameoutput(&user);


    struct Input<'a> {
        argument : Vec<String>,
        path : &'a Path ,

    }


    impl Input <'_> {
        fn collect(&self){
            println!("path {}  , arg 2 {}", &self.path.display() , &self.argument[2] ,);
        }
      
    }

    let value : Vec<String>  =  env::args().collect();

    let  pathexo :&Path = Path::new(&value[1]);

    let foo : Input = Input {argument : value.clone() , path : pathexo };


     foo.collect();

     
}


use std::process::Command;
use std::{self, env} ;



fn main(){

    let value = InitGit {github : true};

    value.deployer();


    struct InitGit {
        github : bool ,
   
    }

    impl InitGit {

        fn deployer(&self){

        
            if self.github {

                let chemin_actuel = env::current_dir().expect("Impossible de lire le dossier actuel");


                 let repo = Command::new("git")
                  .arg("init")
                  .current_dir(&chemin_actuel)
                  .status();


            }
            
        }
    }
}
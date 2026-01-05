use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::string;
use rand::prelude::*;


fn main(){


    let args : Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!(r#"Usage incorrect. Exemple : cargo run -- -path "C:\Dossier" -file "nom1" "nom2""#);
        return;
    }

    let flag = &args[1];
    let path : &String = &args[2];
    let mode  = &args[3];
    let ext : &String = &args[5];
   
    if flag != "-path"{
        println!("The first argument need to be -path");
        
    }else if  flag == "-path" && mode == "-folder" {

        for directory in &args[4..] {
           
           let mut chemin = PathBuf::from(path);
           chemin.push(directory);
        
           match fs::create_dir(&chemin) {
                Ok(_) => println!("✅ Succès"),
                Err(e) => println!("❌ Erreur ({})", e), // Affiche si ça existe déjà
            }
        }

    }else if  flag == "-path" && mode == "-file"{

        let pathtest = Path::new(path);

        if pathtest.exists() {

            for directory in &args[4..]{

                let mut rng = rand::rng();
                let extlist : [&str ; 5] = [".png", ".py" , ".rs", ".jpg", ".txt"];

                let ext_choose = extlist.choose(&mut rng).unwrap();
                let name = format!("{}{}", directory, ext_choose);

                let mut chemin_final = PathBuf::from(path);
                chemin_final.push(name);

                match File::create(&chemin_final) {
                    Ok(_) => println!("✅ Fichiers créés"),
                    Err(e) => println!("❌ Erreur : {}", e),
                }
            }
        }else{
        println!("Path not found problem need to reset");
        }

    }else if path == "-path" && mode == "-exts" {


   
    }

}    


fn randomfile(flag : &String , path : &String , args : &[String] ){



    if flag != "-path"{
        println!("The first argument need to be -path");

    }else{

    
     let pathtest = Path::new(path);

        if pathtest.exists() {

            for directory in &args[4..]{

                let mut rng = rand::rng();
                let extlist : [&str ; 5] = [".png", ".py" , ".rs", ".jpg", ".txt"];

                let ext_choose = extlist.choose(&mut rng).unwrap();
                let name = format!("{}{}", directory, ext_choose);

                let mut chemin_final = PathBuf::from(path);
                chemin_final.push(name);

                match File::create(&chemin_final) {
                    Ok(_) => println!("✅ Fichiers créés"),
                    Err(e) => println!("❌ Erreur : {}", e),
                }
            }
        }else{
        println!("Path not found problem need to reset");
        }
    }

}
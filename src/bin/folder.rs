use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
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
    
   
    match (flag.as_str(), mode.as_str()){

        ("-path", "-random") => randomfile( path, &args),
        ("-path" , "-folder") => multiplefolder(path , &args ),
        _ => println!("Invalid command or argument invalid")

    }


}    


fn randomfile( path : &String , dirs : &[String] ){
    
     let pathcheck = Path::new(path);

        if pathcheck.exists() {

            for directory in &dirs[4..]{

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
            eprintln!("❌ Erreur critique : Le dossier cible '{}' est introuvable.", path);
            eprintln!("👉 Vérifiez que le chemin existe avant de lancer la commande.");
    
            return;
        }
}

fn multiplefolder(path : &String , dirs : &[String]){

    let pathexist = PathBuf::from(path);

    if pathexist.exists(){

    for directory in &dirs[4..] {
           
        let mut road = PathBuf::from(path);
        road.push(directory);
        
        match fs::create_dir(&road) {
            Ok(_) => println!("✅ Succès"),
            Err(e) => println!("❌ Erreur ({})", e), // Affiche si ça existe déjà
        }
    }

    }else{
            eprintln!("❌ Erreur critique : Le dossier cible '{}' est introuvable.", path);
            eprintln!("👉 Vérifiez que le chemin existe avant de lancer la commande.");
    
            return;
    }


}


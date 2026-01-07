use std::fmt::format;

use clap::builder::Str;
use reqwest;

#[tokio::main]
async fn main(){

    struct Serveur {
    protocole : String,
    url: String,
    online : bool,
    }

   impl Serveur {
    // Fonction pour créer le checker
    fn new(protocole : String  , url : String) -> Self {

        Serveur {protocole : String::from(protocole),
                url: String::from(url),
                online: true
             }
    }
    
    fn print(&self) {

        println!("{}://{}.com", &self.protocole , &self.url)

    }
    // Fonction pour ajouter un serveur
    fn ajouter(&self)  -> String {

        let value = format!("https://{}.com" , &self.url );
        return value ;
    }

    }    

    let mut serveur : Serveur = Serveur::new("https".to_string(), "google".to_string());

    let serv = serveur.ajouter();

    async fn verif(value : String){

        let body = reqwest::get(&serv).await().text();

        println!("{body:?}");
    }


    serv.verif();
    
    


}

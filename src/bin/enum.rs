#[derive(Debug)]
enum Weapon {
    Epee ,
    Arc ,
    Baton , 
    MainNue,
    Morgeinsteirn

}



impl Weapon {


    fn weapondamage(&self) -> i32 {

        match self {
            Weapon::Epee => 10,
            Weapon::Arc => 5 ,
            Weapon::Morgeinsteirn => 12 ,
            Weapon::MainNue => 2 ,
            Weapon::Baton => 5
        }
    }
}


// Statistiques du personnage 

 #[derive(Debug)]
struct Stats {
    Vig : i32 ,
    Force : i32 ,
    End : i32 ,
    Int : i32 ,
    Dex : i32 ,
    Luck : i32 
}

impl Stats {

    fn IntStats(&self) -> [i32 ; 6]{
        [self.Vig , self.Force , self.End , self.Int , self.Dex , self.Luck]
    }

    fn StrStats() -> [String ; 6] {
       ["Vigueur".to_string() , "Force".to_string() , "Endurance".to_string(), "Intelligence".to_string(), "Dexterity".to_string() ,"Luck".to_string() ]
    }


   
}
// Role du personnage 

#[derive(Debug)]

enum Role {
    Chevalier ,
    Clerc ,
    Pyromancien,
    Mendiant , 
    Necromancien ,
    Prisonnier
}

impl Role{

    fn basicStats(&self) -> Stats {

    match self {
    Role::Chevalier => Stats { Vig: 12, Force: 14, End: 12, Int: 6, Dex: 10, Luck: 7 },
    Role::Clerc => Stats { Vig: 10, Force: 12, End: 9, Int: 14, Dex: 8, Luck: 11 },
    Role::Pyromancien => Stats { Vig: 11, Force: 11, End: 10, Int: 12, Dex: 12, Luck: 7 },
    Role::Prisonnier => Stats { Vig: 8, Force: 8, End: 11, Int: 14, Dex: 14, Luck: 6 },
    Role::Necromancien => Stats { Vig: 7, Force: 6, End: 8, Int: 16, Dex: 9, Luck: 12 },
    Role::Mendiant => Stats { Vig: 10, Force: 10, End: 10, Int: 10, Dex: 10, Luck: 10 },
    }

    }

    fn Weaponclass(&self) -> Weapon {
        match self {
            Role::Chevalier => Weapon::Epee , 
            Role::Clerc => Weapon::Baton,
            Role::Mendiant => Weapon::MainNue,
            Role::Prisonnier => Weapon::Morgeinsteirn,
            Role::Pyromancien => Weapon::Arc,
            Role::Necromancien => Weapon::Baton

        }
    }

    

      
}


// Personnage création 

struct Personnage {
    hp : i32 , 
    nom : String ,
    classe : Role ,
    arme : Weapon ,
    stats : Stats
    
}

impl Personnage {

    fn new(nom : String , classe : Role) -> Personnage {

        let arme = classe.Weaponclass();
        let stats = classe.basicStats();

        Personnage {
            hp : stats.Vig * 10 ,
            nom : nom , 
            classe : classe,
            arme : arme ,
            stats : stats 
        }

    }

    fn fiche(&self) {
        println!("===============================");
        println!("NOM : {}", self.nom);
        println!("CLASSE : {:?}", self.classe);
        println!("SANTÉ : {} HP", self.hp);
        println!("ARME : {:?}", self.arme);
        println!("DÉGÂTS ARME : {}", self.arme.weapondamage());
        println!("-------------------------------");

        // On appelle la logique de Stats depuis ici
        let noms = Stats::StrStats();
        let valeurs = self.stats.IntStats();

        for (n, v) in noms.iter().zip(valeurs.iter()) {
            println!("{:<12} : {}", n, v);
        }
        println!("===============================");
    }





   
}

fn main(){

    let picard : Personnage = Personnage::new(String::from("Picard") , Role::Chevalier );
    let Firekeeper : Personnage = Personnage::new(String::from("Firekeeper") , Role::Clerc);

    
    Firekeeper.fiche();

    picard.fiche();

    

}

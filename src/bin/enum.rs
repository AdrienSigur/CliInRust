
#[derive(Debug)]

enum Role {
    Chevalier ,
    Clerc , 
    Pyromancien ,
    Prisonnier ,
    Necromancien,
    Mendiant
}
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

    fn StatArray(&self) -> [i32 ; 6]{
        [self.Vig , self.Force , self.End , self.Int , self.Dex , self.Luck]
    }
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

}


struct Personnage {
    hp : i32 , 
    nom : String ,
    classe : Role ,
}




impl Personnage {

    fn new(nom : String , classe : Role) -> Personnage {

        let stat = classe.basicStats();

        Personnage {
            hp : stat.Vig * 10 ,
            nom : nom , 
            classe : classe,
        }

    }

    fn Stats(&self) {

        let charact = &self.classe.basicStats().StatArray();

        for value in charact {
            println!("{:?}" , value );
        }
    }

    fn Hi(&self){
        println!("Bonjour je suis {:?} je m'appelle {} et j'ai {:?} hp", &self.classe , &self.nom , &self.hp );
    }
}


fn main(){
    let picard : Personnage = Personnage::new(String::from("Picard") , Role::Chevalier );

    picard.Hi();

    picard.Stats();
}

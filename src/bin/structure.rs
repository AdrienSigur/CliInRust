#[derive(Debug)]

pub struct User {
    active : bool ,
    username : String ,
    email : String ,
    sign_in_count : u64
}


impl User {

    pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }

    }

    pub fn userprint(&self){
        println!("L'utilisateur a {} connexion son Username est {} et son mail {}" , &self.sign_in_count , &self.username , &self.email);
    }
}




use std::fmt;



//similares a las tuplas, ambas pueden tener varios valores de diferente tipo de dato
// similares a las struct de C o las clases de Typescript o Java
struct User {
    username: String,
    email: String,
    active: bool,
    age: u8
}


//veremos más adelantes que son los Traits
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ username: {}, email: {}, active: {}, age: {} }}", 
        self.username, self.email, self.active, self.age)
    }
}

fn main() {
    //usando una struct

    let user1 = User { 
        username: String::from("berracode"),
        email: String::from("berracode@gmail.com"),
        active: true,
        age: 30
    };

    println!("username {}", user1.username);

    //para modificar algun campo de la estructura, la variable debe ser mutable, rust no permite marcar
    // los campos como mutables

    let mut user2 = User { 
        username: String::from("berracode2"),
        email: String::from("berracode2@nada.com"),
        active: false,
        age: 32
    };

    user2.email = String::from("berracode2@gmail.com");

    println!("email user2 {}", user2.email);

    let user3 = build_user( String::from("user3"),  String::from("user3@gmail.com"), 12);

    //println!("User3 {} ", user3); //error porque User no implementa Display trait

    //solución implementar Display para User

    println!("User3 {} ", user3);

    let user4 = build_user_version_2( String::from("user4"),  String::from("user4@gmail.com"), 233);

    println!("User4 {} ", user4);




}


//función para crear User
fn build_user(username: String, email: String, age: u8) -> User {
    User {
        username: username,
        email: email,
        age: age,
        active: true,
    }
}


//version 2, Field Init Shorthand
fn build_user_version_2(username: String, email: String, age: u8) -> User {
    User {
        username,
        email,
        age,
        active: true,
    }
}
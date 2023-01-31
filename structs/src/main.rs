

//similares a las tuplas, ambas pueden tener varios valores de diferente tipo de dato
// similares a las struct de C o las clases de Typescript o Java
struct User {
    username: String,
    email: String,
    active: bool,
    age: u8
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
}

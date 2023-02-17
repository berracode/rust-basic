use std::env;


enum Command {
    String,
    ListAll,
    Delete,
    Update,
    FindById,  
}

struct Params {
    command: Command,
    todo: Todo

}

impl Params {
    
    fn new(args: &[String])  {

        if args.len() >= 3 {
            let command_arg = args[1];

            if command_arg == String::from("create") {

            } else if command_arg == String::from("list") {

            } else if command_arg == String::from("delete") {

            } else if command_arg == String::from("update") {

            } else if command_arg == String::from("findbyid"){
                
            }

            Ok(Params { command: })
        }

    }
}

struct Todo {
    id: Option<i32>,
    name: String,
    description: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
}



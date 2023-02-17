use std::{env, process};

type DataBase = Vec<Todo>;

fn blank_db() -> DataBase {
    Vec::new()
}


enum Command {
    Create,
    ListAll,
    Delete,
    Update,
    FindById,  
}

#[derive(Default)]
struct Params {
    command: Option<Command>,
    todo: Option<Todo>

}

impl Params {
    
    fn new(args: &[String]) -> Result<Params, &'static str>{

        if args.len() >= 3 {
            let command_arg = args[1].clone();

            let mut params = Params::default();

            if command_arg.eq_ignore_ascii_case("create") {
                params.command = Some(Command::Create); //2 id, 3 name, 4 description
                let todo = Todo{
                    id: args[2].parse().unwrap(),
                    name: args[3].clone(),
                    completed: false,
                };
                params.todo = Some(todo);

            } else if command_arg.eq_ignore_ascii_case("list") {

            } else if command_arg.eq_ignore_ascii_case("delete") {

            } else if command_arg.eq_ignore_ascii_case("update") {

            } else if command_arg.eq_ignore_ascii_case("findbyid"){

            } else {
                // TODO print help
            }

            Ok(params)
        }else {
            return Err("Some arguments not found");

        }

    }
}

struct Todo {
    id: i32,
    name: String,
    completed: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let params = Params::new(&args).unwrap_or_else(|error|{
        eprintln!("Error {error}");
        process::exit(0);
    });



}




fn handle_command(params: Params){
    let mut data = blank_db();

    match params.command {
        Some(Command::Create) =>{
            println!("create");
            data.push(params.todo.unwrap());
        },
        Some(Command::Delete) => println!("delete"), 
        _ => println!("No")
        
    }
}



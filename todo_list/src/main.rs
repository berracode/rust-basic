use std::{env, process, io, fmt::Error, rc::Rc, sync::{Mutex, Arc}, cell::RefCell};

type DataBase = Rc<RefCell<Vec<Todo>>>;



fn blank_db() -> DataBase {
    Rc::new(RefCell::new(Vec::new()))
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



#[derive(Debug, Clone)]

struct Todo {
    name: String,
    completed: bool
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let db = blank_db();
    menu(db);
    
}

fn menu(db: DataBase) {

    loop {
    println!("\nElije la opción: \n");
        println!("1. Crear\n2. Listar\n3. Salir\n");
        let mut opcion =  String::new();

        io::stdin().read_line(&mut opcion).expect("Error");


        match opcion.trim().parse().unwrap() {
            1 =>{
                println!("1");
                handle_command(db.clone(), Command::Create);
            } ,
            2 => {
                println!("2");
                handle_command(db.clone(), Command::ListAll);
            },
            3 => process::exit(0),
            _ => println!("none")     
        }
        println!("big db len: {}", db.borrow().len());
        
    }
}




fn handle_command(db: DataBase,  command: Command){
    let mut data = db.borrow_mut();

    match command {
        Command::Create =>{
            println!("create");
            let todo = create_todo();
            println!("Ingresado");
            data.push(todo);
            println!("db len: {}", data.len());

        },
        Command::ListAll => {
            println!("list here: {}", data.len());
            data.iter().for_each(|todo| {println!("todo {todo:?}")});
            
        }, 
        _ => println!("No")
        
    }
}

fn create_todo() -> Todo {
    let mut name = String::new();
    let mut completed = String::new();

    println!("Nombre del todo: ");
    io::stdin().read_line(&mut name).expect("Error");

    println!("Estado del todo");
    io::stdin().read_line(&mut completed).expect("Error");


    Todo { name: name, completed:  completed.trim().parse().unwrap()}

}



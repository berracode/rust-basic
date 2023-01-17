fn main() {
    //ENTEROS 
    let integer: i32 = 20000;

    // 
    let age: i8 = 12;
    let age: u8 = 255;
    //let age: u8 = 256; //error de desbordamiento. 


    //ENTEROS LITERALES

    let max_value_i32 = 2147483647;
    println!("1. max_value entero: {}", max_value_i32);
    let max_value_i32 = 2_147_483_647;
    println!("2. max_value entero literal: {}", max_value_i32);


    //FLOTANTES (f32 y f64 y ambos son con signo)

    let x = 200000000.0;
    let y: f32 = 3.0;
    let z = 2.0;

    //let w: f32 = 3; //error de coincidencia de tipos, no existe el casting automatico. 

    //FLOTANTES CON SEPARADOR

    let float_with_separator = -11_595_384.230_000;

    println!("float_with_separator {}", float_with_separator);

    // BOOLEAN

    let is_fun: bool = true;
    println!("Is Rust Fun? {is_fun}");

    //CARACTER O CHAR

    let c: char = 'z';
    let d = 'Æ'; //alt 146
    let e = '#';




}

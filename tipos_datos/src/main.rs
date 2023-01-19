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

    // TIPO TUPLA
    let tuple: (i32, f32, bool, char) = (23, 100.09, true, 'A');
    let tuple = (true, false, 6.4);

    //obtener los valores de una tupla del
    let (is_fun, is_bad, cash) = tuple;

    println!("The value of cash is: {cash}");

    let bool_true = tuple.0;
    let bool_false = tuple.1;
    let six_point_four =  tuple.2;

    println!("The value of six_point_four is {six_point_four}");

    //ARRAY

    


}

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

    let notas_academicas =  [2.3, 2.1, 3.1, 3.3, 3.4]; //array de 5 posiciones de numeros flotantes de 64 bits 

    let frutas = ["Manzana", "pera", "banano", "maracuya"];

    //para declarar sin dejar que la infiera rust
    let marcas_automoviles: [&str; 3] = ["Ferrari", "Toyota", "BMW"];

    //mismo valor para todos los elementos

    let array_de_10 = [10; 3];
    //equivalente a escribir

    let array_de_10 = [10, 10, 10];

    //acceso a elementos del array
    let enteros = [10, 20, 30, 40, 50];
    println!("enteros en la posición 0 ={}", enteros[0]);
    println!("enteros en la posición 1 ={}", enteros[1]);
    println!("enteros en la posición 2 ={}", enteros[2]);
    println!("enteros en la posición 3 ={}", enteros[3]);
    println!("enteros en la posición 4 ={}", enteros[4]);



}

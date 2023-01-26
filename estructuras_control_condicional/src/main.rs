fn main() {
    // expresiones if

    let is_rust_fun = true;

    if is_rust_fun == true {
        println!("Rust is fun");
    }else{
        println!("Rust is not fun")
    }

    let number = 101;

    if number != 10 {
        println!("El numero no es 10");
    }else {
        println!("El numero es 10")
    }


    let numero  =  3;

    if (numero % 4) == 0 {
        println!("el numero es divisible por 4");
    } else if numero % 3 == 0 {
        println!("el numero es divisible por 3");
    } else if numero % 2 == 0{
        println!("el numero es divisible por 2");
    } else {
        println!("el numero no es divisible ni por 4, 3 o 2");
    }

    // if en una declaración let

    let condicion = true;
    let numero  = if condicion {5} else {6};

    println!("el valor del numero es: {numero}");


}

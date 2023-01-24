fn main() {
    //repetición con loop - ejecuta para siempre un bloque de código

    /*loop {
        println!("Otra vez!");
    }*/

    //devolviendo valores de un loop 


    let mut contador = 0;

    let resultado = loop {
        contador += 1;

        if contador == 5 {
            break contador*2;
        }
    };

    println!("El resultado es: {}", resultado);


}

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

    //loop copn etiquetas o labels

    let mut conteo = 0;

    'contando: loop {
        println!("Conteo: {conteo}");

        let mut restante = 10;

        loop {
            println!("Restante: {restante}");
            if restante == 9 {
                break;
            }

            if conteo == 2 {
                break 'contando;
            }

            restante-=1;
        }

        conteo += 1;

    }

    println!("Conteo final: {}", conteo);

    println!("El resultado es: {}", resultado);


    //bucles con while

    let mut incremental = 0;

    while incremental < 10 {

        println!("incremental {}", incremental);

        incremental += 1;
        
    }


    // recorriendo array

    let enteros = [3,5,9,90,100];

    for elemento in enteros {
        println!("elemento {}", elemento);
        
    }


}

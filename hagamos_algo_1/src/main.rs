fn main() {
    let enteros = [73 , 117 , 987 , 313 , 185 , 697 , 526 , 69 , 995 , 998 , 815 , 889 , 
    240 , 143 , 213 , 299 , 738 , 212 , 140 , 74 , 834 , 460 , 823 , 889 , 591 , 869 , 648 , 651 , 116 ,
    493 , 982 , 411 , 16 , 473 , 619 , 301 , 724 , 698 , 689 , 456 , 193 , 899 , 642 , 704 , 414 , 37 , 
    955 , 872 , 445 , 201 , 201 , 644 , 610 , 293 , 751 , 859 , 110 , 39 , 829 , 280 , 160 , 104 , 344 , 
    930 , 364 , 974 , 937 , 230 , 165 , 391 , 535 , 373 , 110 , 389 , 126 , 818 , 682 , 438 , 119 , 517 , 
    241 , 841 , 336 , 466 , 471 , 765 , 198 , 477 , 498 , 339 , 139 , 818 , 308 , 83 , 9 , 24 , 638 , 714 , 4 , 530];

    cantidad_enteros_pares(enteros);
    cantidad_enteros_impares(enteros);

}

fn cantidad_enteros_pares(enteros: [i32; 100]){
    let mut contador_enteros = 0;
    for entero in enteros {
        if entero % 2 == 0 {
            contador_enteros+=1;
        }
    }

    println!("cantidad de enteros pares: {contador_enteros}");

}

fn cantidad_enteros_impares(enteros: [i32; 100]){
    let mut contador_enteros_impares = 0;
    for entero in enteros {
        if entero % 2 == 1 {
            contador_enteros_impares+=1;
        }
    }

    println!("cantidad de enteros impares: {contador_enteros_impares}");

}

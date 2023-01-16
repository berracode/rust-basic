fn main() {
    // ENTEROS y ENTEROS LITERALES

    let age: u8 = 78;
    println!("el valor de age es: {age}");

    let max_value: i32 = 2_147_483_647;
    println!("max_value {max_value}");

    //esto da error, hemos sobrepasado el rango
    //let max_value_plus_one: i32 = 2_147_483_648;
}

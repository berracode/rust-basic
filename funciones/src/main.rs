

fn main() {

    let variable_entera: i32 = 23;

    println!("vairable entera {}", variable_entera);

    otra_funcion();
    una_funcion_con_parametros(variable_entera);
    una_funcion_con_varios_parametros(12, 'J', 1.80);
    let x = una_funcion_con_retorno();
    println!("valor de x = {x}");

    let x = una_funcion_con_retorno_y_parametro(5);
    println!("el nuevo valor de x es = {x}");
}

/// función sin parametros y sin retorno
fn otra_funcion() {
    println!("Otra función");
}

fn una_funcion_con_parametros(parametro: i32) {
    println!("valor del parametro = {}", parametro);
}

fn una_funcion_con_varios_parametros(edad: u8, inicial_nombre: char, estatura: f32) {
    println!("edad = {}", edad);
    println!("inicial_nombre = {}", inicial_nombre);
    println!("estatura = {}", estatura);
}

fn una_funcion_con_retorno() -> i32 {
    256
}

fn una_funcion_con_retorno_y_parametro(x: i32) -> i32 {
    x*x
}

fn una_funcion_con_retorno_y_parametro_multilinea(x: i32) -> i32 {
    let y = (x*x) + 1;
    println!("valor de y es = {y}");
    return y;
}

fn una_funcion_con_retorno_y_parametro_multilinea_version_2(x: i32) -> i32 {
    let y = (x*x) + 1;
    println!("valor de y es = {y}");
    //y; //error
    y
}
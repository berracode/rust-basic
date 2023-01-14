fn main() {

   let mut letras = "hola mundo";

   //letras = letras.len(); error

   println!("letras {letras}");


   let mut number = 10;
   println!("el valor de number es: {number}");
   number = 10+1;
   println!("el valor de number es: {number}");

   let x = 5;
   println!("el valor de x es: {x}");
   let x = x + 1;

   {
      let x = x*2;
      println!("el valor de x es: {x}");
   }

   println!("el valor de x es: {x}");



  
}

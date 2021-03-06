/* Tipos de datos, desbordamiento, conversión de tipos, errores frecuentes
*/

use std::io;

fn main() {
    println!("Introduce un número");
    let mut numero_cadena = String::new();
    io::stdin().read_line(&mut  numero_cadena).expect("Error al obtener el número");
    println!("El siguiente número se ha guardado como cadena: {}", numero_cadena);

    /* Intentamos sumar 7 al múmero inical, pero no podemos, por que el número original es
    un string
    let mut numero = numero_cadena + 7 ;
    hay que "convertirlo", y  cuando se recibe entrada por terminal, la f(x) trim es importante
    para que no haya errores con espacios ni saltos de línea*/

    let numero: u8 = numero_cadena.trim().parse().expect("No es un número");
    let  resultado = numero + 7;
    println!("El siguiente número se ha guardado como unsigned de 8 bits: {}", resultado);

    /* Al guardarlo como unsigned de 8 bits, el valor máximo que admite es de 0 a 255.
    probamos a cambiarlo a unsigned de 16 bits. En este ejemplo usamos shadowing
    */
    let resultado: u16 = 15645;
    println!("El siguiente número se ha guardado como unsigned de 16 bits: {}", resultado);

    /* Ahora vamos a realizar un ejemplo con variable as...*/
    // let resultado = resultado as u32;
    println!("El siguiente número se ha guardado como unsigned de 32 bits: {}", resultado as u32);

}

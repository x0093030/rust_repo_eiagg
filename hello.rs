// Este es un programa básico en Rust

fn main() {
    // Imprime un mensaje en la consola
    println!("¡Hola desde Rust!");

    // Declaración de variables
    let x = 42;
    let nombre = "Usuario";

    // Uso de formateo de strings
    println!("Hola, {}! El número es: {}", nombre, x);

    // Un ejemplo de un loop simple
    for i in 1..5 {
        println!("Número: {}", i);
    }

    // Un ejemplo de uso de condiciones
    let numero = 7;
    if numero > 5 {
        println!("El número es mayor que 5");
    } else {
        println!("El número es menor o igual a 5");
    }
}

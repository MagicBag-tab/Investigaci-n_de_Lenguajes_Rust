//! Ejemplo de estrucutras de datos
//! 
//! Cadenas 
//! Link de referencia para el ejemplo: 
//! https://www.youtube.com/watch?v=QYLe-7-vKgk&list=PLAMfQH2NKM_tyKzBV1iJf5L8j7oJl6KHl&index=15

fn main() {
    
    let cadena = String::new(); //definicion de una cadena mutable vacía

    let datos = "Hola Mundo!";
    let cadena = datos.to_string(); //convierte en string

    let cadena2 = "Hola Mundo!".to_string(); //otra forma de convertir a string

    let cadena3 = String::from("Hola Mundo!"); //otra forma
    let cadena_japones = String::from("こんにちは世界！"); //puede ver otro tipo de caracteres
    let cadena_emoji = String::from("💕🦋🐞"); //puede poner emoticones 

    let nombre = String::from("Sarah");
    let apellido = String::from("Estrada");

    let nombre_apellido = nombre + &apellido; //concatenar dos Strings
    println!("{}", cadena_japones);
    println!("{}", cadena_emoji);
    println!("{}", nombre_apellido);

}
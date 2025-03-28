use std::{cmp::Ordering, io};

use rand::Rng; //Importa la libreria de rand en rust

fn main() {
    

    let secreto: u32 = rand::thread_rng().gen_range(1..=30); //genera el numero secreto

    loop{ //el ciclo donde el usuario juega hasta adivinar
        println!("Hola 😊, introduce un valor de 1 a 30");

        let mut guess = String::new(); //variable donde se guarda que adivina el usuario

        io::stdin().read_line(&mut guess).expect("Existe un error al leer 😔"); //lee la respuesta del usuario

        println!("El valor introducido es: {guess}"); //muestra el valor que el usuario ingreso

        let guess: u32 = guess.trim().parse().expect("Introduce un entero porfavor"); //vuelve el valor introducido en entero

        match guess.cmp(&secreto) { //copara el valor ingresado con el generado
            Ordering::Less => println!("El valor es menor que el generado 🫠"),
            Ordering::Greater => println!("El valor es mayor al generado 🥺"),
            Ordering::Equal => {
                println!("Haz adivinado 🎆🎆🎆🎊🎊🎊✨✨✨🎉🎉🎉🧚‍♀️");
                break; //se sale del programa
            } 
        }
    }
}


//puedo hacer más mejoras, como una opción de rendirse y ver el numero, una excepcion si el usuario ingresa un valor 
//que no este en el rango o si desea volver a jugar con un nuevo numero.
//tutorial utilizado: https://www.youtube.com/watch?v=Ho2thWi8BuA
//si algo me gusta es poner muchos emojis.
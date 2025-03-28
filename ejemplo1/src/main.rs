//! Ejemplo de las estructuras de datos en Rust
//! 
//! Vectores 
//! Link tutorial tomado en cuenta para este ejemplo: 
//! https://www.youtube.com/watch?v=lkP6D2YtBdI&list=PLAMfQH2NKM_tyKzBV1iJf5L8j7oJl6KHl&index=14
fn main() {

    let mut v: Vec<i32> = Vec::new(); //definir un tipo de vector
    v.push(5); //añade elementos a un vector
    v.push(2);
    v.push(1);

    println!("La longitud del vector es: {}", v.len());

    v.pop(); //elima el ultimo elemento del vector
    println!("La longitud del vector es: {}", v.len());

    let mut v2 = vec![1, 2, 3, 4, 5, 6, 7, 8]; //inicializando un vector

    let cuarto: &i32 = &v2[3]; //acceder a un elemento
    println!("El cuarto elemento es: {}", cuarto);

    match v2.get(4){ //otra forma de acceder a un elemento
        Some(cuarto) => println!("Si coincide con el cuarto valor"),
        None => println!("No coincide"),
    }

    //Para ver todo el vector iterandolo
    for i in &v2{
        print!(" {}", i);
    }

    //modificar v2 mientras esta iterando
    for i in &mut v2 {
        *i += 10; //le suma 10 al valor del vector
        print!(" {}", i);
    }


}

//! Utilizando tipos de datos y estructuras de datos, chat sugirió realizar un programa
//! para poder ordenar las notas de estudiantes en vevtores con tipo de dato punto
//! flotante. Se tomará en cuenta los ejemplos realizados para manejar los vectores y las cadenas.

fn main() {

    println!("📓 Notas de Estudiantes");
    //Estudiantes utilizando cadenas String para guardar los nombres en un vector
    let nombre_estudiantes = vec![String::from("Sarah"), String::from("Meredith")];

    //Notas de los estudiantes utilizando vectores que guardan estas en tipo punto flotante
    let notas = vec![vec![60.5, 70.8, 20.9, 80.5, 30.5], vec![29.4, 90.2, 48.7, 51.6, 100.0]];

    // itera los nombres, calificaciones y promedio del estudiante.
    for i in 0..nombre_estudiantes.len() {
        let nombre = &nombre_estudiantes[i];
        let nota = &notas[i];

        let suma: f32 = nota.iter().sum();
        let promedio = suma / nota.len() as f32;

        println!("\n 👩‍🦱 Estudiante: {}" , nombre);
        println!("📝 Notas: {:?}", nota); //:? para tener todos los calores
        println!("🏫 Promedio de Notas: {:.1}", promedio); //:.1 indica que quiero 1 decimnal
    }
}

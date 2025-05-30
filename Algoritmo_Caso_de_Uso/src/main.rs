//Algoritmo de Búsqueda Binaria en Rust como ejemplo de caso de uso
// Este algoritmo busca un elemento en un arreglo ordenado utilizando el método de búsqueda binaria.
// La función recibe un slice de enteros y un entero objetivo, y devuelve la posición del elemento si se encuentra, o None si no.
// El algoritmo funciona dividiendo el arreglo en mitades y comparando el elemento objetivo con el elemento del medio.
// El tiempo de ejecución es O(log n), lo que lo hace eficiente para arreglos grandes.


fn binary_search(arr: &[i32], target: i32) -> Option<usize> {       //Función que realiza la búsqueda binaria
    
    // Verifica si el arreglo está vacío
    if arr.is_empty() {
        return None;
    }
    // Inicializa los índices izquierdo y derecho
    let mut left = 0;
    let mut right = arr.len();

    // Bucle para dividir el arreglo hasta encontrar el elemento o agotar las posibilidades
    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),      // Si el elemento del medio es igual al objetivo, devuelve su índice
            std::cmp::Ordering::Less => left = mid + 1,         // Si el elemento del medio es menor que el objetivo, busca en la mitad derecha
            std::cmp::Ordering::Greater => right = mid,         // Si el elemento del medio es mayor que el objetivo, busca en la mitad izquierda
        }
    }
    None
}

// Ejemplo de uso de la función binary_search
fn main() {
    // Arreglo ordenado y elemento a buscar
    // El arreglo debe estar ordenado para que la búsqueda binaria funcione correctamente
    // El elemento objetivo debe ser un entero que se busca en el arreglo
    // El resultado será la posición del elemento si se encuentra, o None si no está presente
    let sorted_array = [1, 3, 5, 7, 9, 11, 13];
    let target = 7;

    // Llamada a la función binary_search y manejo del resultado
    // Se imprime el resultado de la búsqueda, indicando si el elemento fue encontrado y su posición
    // Si el elemento no se encuentra, se imprime un mensaje indicando que no fue encontrado
    println!("Buscando el elemento {} en el arreglo {:?}", target, sorted_array);
    // Llamada a la función binary_search con el arreglo ordenado y el elemento objetivo
    match binary_search(&sorted_array, target) {     // Se utiliza un match para manejar el resultado de la búsqueda
        Some(index) => println!("Elemento encontrado en la posición {}", index),
        None => println!("Elemento no encontrado"),
    }
}

//Puedes implementar otros algoritmos como Dijkstra para comprobar su eficiencia
# Estructuras de Datos en Rust

Este proyecto en Rust implementa un gestor de tareas en línea de comandos.
Utiliza estructuras como Vec<Task> para el manejo dinámico de las tareas,
enum Action para representar las acciones posibles, y serde_json para la
serialización de datos.
También se emplean tipos como DateTime<Utc> y PathBuf para manejo de fechas y archivos.

# Uso

## Para añadir una tarea utiliza

cargo run -- add "Tarea a agregar"

## Listar tareas

cargo run -- list

## Completar una tarea

cargo run -- done IdTarea(por ejemplo 1)

cargo run -- done 1

## Si quieres otro archivo diferente al predeterminado utiliza

cargo run -- -j nueva_lista.json add "Tarea Nueva"

cargo run -- -j nueva_lista.json list

cargo run -- -j nueva_lista.json done 1


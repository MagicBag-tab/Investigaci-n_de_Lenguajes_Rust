#Uso

##Para añadir una tarea utiliza

cargo run -- add "Tarea a agregar"

##Listar tareas

cargo run -- list

##Completar una tarea

cargo run -- donde IdTarea(por ejemplo 1)

cargo run -- donde 1

##Si quieres otro archivo diferente al predeterminado utiliza

cargo run -- -j nueva_lista.json add "Tarea Nueva"

cargo run -- -j nueva_lista.json list

cargo run -- -j nueva_lista.json done 1


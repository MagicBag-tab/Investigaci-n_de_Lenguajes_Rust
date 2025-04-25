use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    Add { 
        #[structopt()]
        task: String,
    }, //agrega la tarea
    
    Done { 
        #[structopt()]
        position: usize, 
    }, //terminar la tarea

    List, //listado de tareas
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "toDo List Journal",
    about = "Lista de tareas"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>, //archivo
}
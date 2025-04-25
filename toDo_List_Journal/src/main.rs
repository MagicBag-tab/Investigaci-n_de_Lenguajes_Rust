mod cli;  //modulo diferente para organizar mejor el programa
mod tasks; //modulo de tareas

use structopt::StructOpt;
use cli::{Action:: *, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;

fn find_default_journal_file() -> Option<PathBuf>{
    home::home_dir().map(|mut path| {
        path.push("mi-diario.json");
        path
    })
}

fn main() {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
    .or_else(find_default_journal_file)
    .expect("No se encontr el archivo");

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("No se pudo realizar la acción")
}

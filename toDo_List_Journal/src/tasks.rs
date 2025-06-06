use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;

use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};

use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,

}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now(); //asiga la fecha de creación
        Task { text, created_at } //la tarea tiene texto y la fecha
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H: %M");
        write! (f, "{:<50} [{}]", self.text, created_at) //guarda hasta 50 caracteres de la tarea y la fecha
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()>{
    //abre el archivo
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    //vuelve el contenido del archo en vectores para las tareas
    let mut tasks = collect_task(&file)?;
    tasks.push(task); //agrega la tarea
    serde_json::to_writer(file, &tasks)?; //vuelve a escribir el archivo

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(journal_path)?;

    let mut tasks = collect_task(&file)?;

    //Remover una tarea
    if task_position == 0 || task_position > tasks.len()  {
        return Err(Error::new(ErrorKind::InvalidInput, "Id de la tarea inválido"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?; //vuelve a escribir el archivo

    Ok(())
}

pub fn list_tasks(journal_path:PathBuf) -> Result<()> {

    let file = OpenOptions::new()
        .read(true)
        .open(journal_path)?;

    let tasks = collect_task(&file)?;

    if tasks.is_empty(){
        println!("La lista de tareas esta vacía")
    } else {
        let mut order: u32 = 1;
        //imprime el numero de la tarea y la tarea
        for task in tasks{
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}

fn collect_task(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file){
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(), //error de entrada o salida
        Err(e) => Err(e)?,
    };

    file.seek(SeekFrom::Start(0))?;
    Ok(tasks)
}


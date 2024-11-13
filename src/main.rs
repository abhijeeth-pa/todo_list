
#![feature(decl_macro)]
use rocket::{get, post, delete, routes};
use std::sync::{Mutex};



extern crate rocket;

use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;  // Use StaticFiles instead of FileServer

use crate::tasks::{Task, TodoList};

mod tasks;

#[post("/add", data = "<task>")]
fn add_task(todo_list: State<Mutex<TodoList>>, task: Json<Task>) -> Json<&'static str> {
    let mut list = todo_list.lock().unwrap();
    list.tasks.push(task.into_inner());
    Json("Task added")
}

#[get("/tasks")]
fn list_tasks(todo_list: State<Mutex<TodoList>>) -> Json<Vec<Task>> {
    let list = todo_list.lock().unwrap();
    Json(list.tasks.clone())
}

#[post("/complete/<index>")]
fn complete_task(todo_list: State<Mutex<TodoList>>, index: usize) -> Json<&'static str> {
    let mut list = todo_list.lock().unwrap();
    if let Some(task) = list.tasks.get_mut(index) {
        task.completed = true;
        Json("Task completed")
    } else {
        Json("Task not found")
    }
}

#[delete("/delete/<index>")]
fn delete_task(todo_list: State<Mutex<TodoList>>, index: usize) -> Json<&'static str> {
    let mut list = todo_list.lock().unwrap();
    if index < list.tasks.len() {
        list.tasks.remove(index);
        Json("Task deleted")
    } else {
        Json("Task not found")
    }
}

fn main() {
    rocket::ignite()
        .manage(Mutex::new(TodoList::new()))
        .mount("/", routes![add_task, list_tasks, complete_task, delete_task])
        .mount("/", StaticFiles::from("static"))  // Serve static files
        .launch();
}

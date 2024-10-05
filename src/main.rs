use std::io::{BufReader};
use std::fs::File;
use std::env;
use std::path::Path;
use serde::{Deserialize, Serialize};

// struct for tasks
#[derive(Serialize, Deserialize, Debug)]
struct Task{
	id: usize,
	description: String,
	completed: bool,
}

fn main(){
	let args: Vec<String> = env::args().collect();
	
	// Match user commands (add, list, complete, etc.)
	match args[1].as_str(){
		"help" => {help();},
		"add" => {add(&args[2..].join(" "));},
		"list" => {list();},
		"complete" => {complete(args[2].parse().unwrap());},
		"remove" => {remove(args[2].parse().unwrap());},
		"clear" => {clear();},
		_ => {println!("Unknown command. Use 'todo help' for a list of commands.");},
	}
}

fn help(){
	println!("Commands:");
    println!("  add <description>  Add a new task");
    println!("  list               List all tasks");
    println!("  complete <number>  Mark a task as completed");
    println!("  remove <number>    Remove a task");
    println!("  clear              Remove all tasks");
    println!("  help               Show this help message");
}

// add a task to the to-do list
fn add(description: &str){
	let mut tasks = read_tasks("tasks.json");
	let num = tasks.len() + 1;
	tasks.push(Task { id: num, description: description.to_string(), completed: false });
	write_tasks("tasks.json", &tasks);
}

// list the tasks on the to-do list
fn list(){
	let tasks = read_tasks("tasks.json");
	if tasks.is_empty() {
		println!("No tasks.");
	} else{
		for task in tasks{
			print!("{}. ", task.id);
			if task.completed {
				print!("[X]");
			}else{
				print!("[ ]");
			}
			println!(" {}", task.description);
		}
	}
}

// complete a task on the to-do list
fn complete(num:usize){
	let mut tasks = read_tasks("tasks.json");
	for task in tasks.iter_mut(){
		if task.id == num {
			task.completed = true;
			break;
		}
	}
	write_tasks("tasks.json", &tasks);
}

// remove a task from the to-do list
fn remove(num:usize){
	let mut tasks = read_tasks("tasks.json");
	tasks.retain(|task| task.id != num);
	write_tasks("tasks.json", &tasks);
}

// remove all tasks from the to-do list
fn clear(){
	let tasks = Vec::new();
	write_tasks("tasks.json", &tasks);
}

// read tasks from a json file to a vector of Tasks
fn read_tasks(file_path: &str) -> Vec<Task>{
	let path = Path::new(file_path);
	if !path.exists(){
		return vec![];
	}
	
	let file = File::open(path).expect("Could not open file.");
	let reader = BufReader::new(file);
	
	serde_json::from_reader(reader).unwrap_or_else(|_| {
        println!("Could not parse JSON. Returning empty task list.");
        vec![]
    })
}

// write from a vector of tasks to a json file
fn write_tasks(file_path: &str, tasks: &Vec<Task>){
	let file = File::create(file_path).expect("Could not create file.");
	serde_json::to_writer_pretty(file, tasks).expect("Could not write JSON.");
}
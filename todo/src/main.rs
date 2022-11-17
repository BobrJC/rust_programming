use std::fs::{OpenOptions, File, rename};
use std::io::{self, Write, BufReader, BufRead, Error};
use chrono::{FixedOffset, prelude::{Local, DateTime}};

#[macro_use] extern crate prettytable;
use prettytable::{Table};

#[derive(Debug)]
struct Task {
    name: String,
    date: DateTime<FixedOffset>,
    importance: String,
    text: String,
    done: bool,
}

impl Task {
    fn write_task(&self, file: &mut File) -> io::Result<()>{
        writeln!(file, "{} | {} | {} | {}", self.name, self.date, self.importance, self.text)?;
        Ok(())
    }
}

struct command {
    name: String,
    args: String,
}

fn build_task(task_vec: Vec<String>) -> Task {
    
    Task { 
        name : task_vec[0].clone(), 
        date : DateTime::parse_from_str((task_vec[1].clone())
                                                    .as_str()
                                                    , "%Y-%m-%d %H:%M:%S %z").unwrap(), 
        importance : task_vec[2].clone(),                                                 
        text : task_vec[3].clone(),
        done : false,
        }
}        

fn build_command(command_str: String) {
     command_name = get_first_word(&command_str);
}

fn write_tasks_vec(file: &mut File, tasks_vec: &Vec<Task>) -> io::Result<()> {
    for task in tasks_vec {
        task.write_task(file)?;
    }
    Ok(())
}

fn get_first_word(command: &String) -> String {

    let mut command_name = String::from("");
    if !command.contains(" ") {
        return command.clone()
    }
    for (i, char) in command.chars().enumerate() {
        if char.is_ascii_whitespace()  {
            command_name = command_name + &command[..i];
            break;
        }
    }
    command_name
}
/* 
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
 */
fn read_from_file(file_in: &File) -> Result<Vec<Task>, Error> {
    let mut tasks_vec: Vec<Task> = Vec::new();
    let bufread = BufReader::new(file_in);
    for line in bufread.lines() {
        tasks_vec.push(build_task(line.expect("could not read from file")
                                          .trim_end()
                                          .split(" | ")
                                          .map(String::from)
                                          .collect::<Vec<String>>()));
    }
    Ok(tasks_vec)
}

fn print_table(table : &Table) {
    if !table.is_empty() {
        table.printstd();
    }
    else {
        println!("Table is empty");
    }
}

fn main() {
    println!("Please, enter the command. For help use \"help\"");
    let path: &str = "tasks.txt";
    
        let file_in = OpenOptions::new()
                                    .write(true)
                                    .read(true)
                                    .create(true)
                                    .open(path)
                                    .unwrap();
    let mut tasks_vec = read_from_file(&file_in).unwrap();
    drop(file_in);

    tasks_vec.sort_by(|a, b| a.date.cmp(&b.date));
    
    let mut file_out = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .append(false)
                                    .open(path.to_string() + ".temp")
                                    .unwrap();
                                    
    loop {

        let mut command: String = String::new();
        io::stdin().read_line(&mut command)
                   .expect("Couldn't read line.");
        command = command.trim_end().to_string();
        let mut command_vec = command.clone()
                                                 .split(" | ")
                                                 .map(String::from)
                                                 .collect::<Vec<String>>();
        let command_name = get_first_word(&command);
        command_vec[0] = command_vec[0].replace((command_name.clone() + " ").as_str(), "");
        
        match command_name.as_str() {
            "help" => println!("Allowed commands:
add <name> | <date> | <type> | <text> - adds new task in task list. Date format: YYYY-MM-DD HH:MM:SS. Types can be: fast, usual, long.
do <name> - mark task as done.
del <name> - deletes taks from task list.
list - view all tadks.
edit <name> | <field's numbers> - edit task. Field's numbers can be 1 for name, 2 for date, 3 for type, 4 for text.
exit - exit from application."),

            "add" => {

                command_vec[1] = command_vec[1].clone() + " " + &Local::now().offset().to_string();
        
                tasks_vec.push(build_task(command_vec));
            },
            "do" => {
                tasks_vec.remove(tasks_vec.iter()
                .position(|x| x.name == command_vec[0])
                .expect("Task not found!"));
            },

            "del" => {
                tasks_vec.remove(tasks_vec.iter()
                                                .position(|x| x.name == command_vec[0])
                                                .expect("Task not found!"));
            },
            "list" => {

                let mut fast_table = Table::new();
                let mut usual_table = Table::new();
                let mut long_table = Table::new();
                let mut tasks_table = Table::new();

                for task in &tasks_vec {
                    if task.date > Local::now() {
                        tasks_table.add_row(row![task.name, task.date.naive_local(), task.importance, task.text]);
                    }
                    match task.importance.as_str() {
                        "fast" => fast_table.add_row(row![task.name, task.date.naive_local(), task.importance, task.text]),
                        "usual" => usual_table.add_row(row![task.name, task.date.naive_local(), task.importance, task.text]),
                        "long" => long_table.add_row(row![task.name, task.date.naive_local(), task.importance, task.text]),
                        &_ => break,
                    };
                }
                match command_vec[0].as_str() {
                    "list" => {
                        print_table(&tasks_table);
                    }
                    "all" => {
                        print_table(&fast_table);
                        print_table(&usual_table);
                        print_table(&long_table);
                    },
                    "fast" => {
                        print_table(&fast_table)
                    },
                    "usual" => {
                        print_table(&usual_table);

                    },
                    "long" => {
                        print_table(&long_table);
                    }
                    &_ => println!("Bad table type")
                
                }
            
            },

            "edit" => {
                
                let position: usize = tasks_vec.iter()
                                               .position(|x| x.name == command_vec[0])
                                               .expect("Couldn't find task.");
                for i in command_vec[1].split(' ') {
                    let mut new_val: String = String::new();
                    match i.trim_end() {
                        "1" => {
                            println!("Enter changed name:");
                            io::stdin().read_line(&mut new_val).expect("Couldn't read line.");
                            new_val.pop();
                            tasks_vec[position].name = new_val;
                        },
                        "2" => {
                            println!("Enter changed date:");
                            io::stdin().read_line(&mut new_val).expect("Couldn't read line.");
                            new_val.pop();
                            tasks_vec[position].date = DateTime::parse_from_str((new_val + &Local::now()
                                                                                                    .offset()
                                                                                                    .to_string())
                                                                                                    .as_str(), "%Y-%m-%d %H:%M:%S%z")
                                                                                                    .unwrap();
                        },
                        "3" => {
                            println!("Enter changed importance:");
                            io::stdin().read_line(&mut new_val).expect("Couldn't read line.");
                            new_val.pop();
                            tasks_vec[position].importance = new_val;
                        },
                        "4" => {
                            println!("Enter changed text:");
                            io::stdin().read_line(&mut new_val).expect("Couldn't read line.");
                            new_val.pop();
                            tasks_vec[position].text = new_val;
                        },
                        &_ => {
                            println!("Bad field's number!");
                        },
                    }
                }
            },

            "exit" => break,
            _ => println!("Unknown command, type \"help\" for help."),
        };
    }
    write_tasks_vec(&mut file_out, &tasks_vec).expect("Couldn't write tasks list.");
    rename(path.to_string() + ".temp", path).unwrap();
    
}

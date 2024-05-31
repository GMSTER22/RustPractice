use core::{fmt, task};
use std::{default, io::{self, Write}};

// Task Manager Class
struct TaskManager {

    tasks: Vec<Task>

}

// Task Manager Methods
impl TaskManager {
    
    // Method to add a new task to the tasks collection
    fn add_task(&mut self, task: Task ) {

        self.tasks.push(task);

    }

    // Method to view collection's tasks
    fn view_tasks(&self) {

        let task_length = self.tasks.len();

        if task_length > 0 {

            println!();
    
            for task in &self.tasks {
                println!("{}_ {} ---> {} ", task.id, task.name, task.status);
            }
    
            println!();

        } else {

            println!("No current task. To add a new task select option 3");

        }


    }

    // Method to view a specific task
    fn view_task(&self) {

        let mut task_id_input: String = String::new();

        print!("Enter task id: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut task_id_input)
            .expect("Failed to read line");

        let task_id_input: u32 = task_id_input.trim().parse().expect("Please Enter the Task Id");

        let index = &self.tasks.iter().position(|task| task.id == task_id_input).unwrap();

        let task = &self.tasks[*index];

        println!("{}_ {} ---> {}", task.id, task.name, task.status);

    }

    // Method to add a new task
    fn add_new_task(&mut self) {

        let task_id: u32 = self.tasks.len() as u32;

        let mut task_name_input: String = String::new();

        print!("Enter task name: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut task_name_input)
            .expect("Failed to read line");

        let task = Task {

            id: task_id + 1,
    
            name: String::from(task_name_input.trim()),
    
            status: TaskStatus::Incomplete
    
        };

        self.tasks.push(task);

        println!("Task added!");

    }

    // Method to edit task name or toggle status
    fn edit_task(&mut self) {

        let mut task_id_input: String = String::new();

        print!("Enter task id: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut task_id_input)
            .expect("Failed to read line");

        let task_id_input: u32 = task_id_input.trim().parse().expect("Please Enter the Task Id");

        let index = &self.tasks.iter().position(|task| task.id == task_id_input).unwrap();
        
        let target_task: &mut Task = &mut self.tasks[*index];

        println!("\nPick the property to update:");
        println!("1. Task name");
        println!("2. Task status\n");

        let mut task_property_input: String = String::new();

        print!("Pick option: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut task_property_input)
            .expect("Failed to read line");

        let task_property_input: u32 = task_property_input.trim().parse().expect("Please Enter the Task Property");

        match task_property_input {

            1 => edit_task_name(target_task),

            2 => edit_task_status(target_task),

            _default => println!("Wrong Option, Try Again")

        }

    }
    
    // Method to delete a task
    fn delete_task(&mut self) {

        let task_length = self.tasks.len();

        if task_length > 0 {

            let mut task_id_input: String = String::new();

            print!("Enter task id: ");

            let _ = io::stdout().flush();

            io::stdin()
                .read_line(&mut task_id_input)
                .expect("Failed to read line");

            let task_id_input: u32 = task_id_input.trim().parse().expect("Please Enter the Task Id");

            let index = &self.tasks.iter().position(|task| task.id == task_id_input).unwrap();

            self.tasks.remove(*index);

            println!("Task #{} deleted", task_id_input);

        } else {

            println!("Currently no task to be deleted");

        }

    }

}

// Function that takes a task and edit its name value
fn edit_task_name(task: &mut Task) {

    let mut task_name_input: String = String::new();

    print!("Enter the new task name: ");

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut task_name_input)
        .expect("Failed to read line");

    task_name_input = task_name_input.trim().to_string();

    task.change_task_name(task_name_input);

    println!("Task {} has been updated", task.id);

}

// Function that takes a task and toggle its status value
fn edit_task_status(task: &mut Task) {

    if task.status == TaskStatus::Complete { 
        
        task.change_task_status(TaskStatus::Incomplete);
    
    } else {

        task.change_task_status(TaskStatus::Complete);

    }

    println!("Task status has been updated");

}

// TaskStatus Enum
#[derive(PartialEq)]
enum TaskStatus {

    Complete,
    Incomplete

}

// Task class for task instances creation
struct Task { 
    
    id: u32,

    name: String,

    status: TaskStatus 

}

// Task methods
impl Task {
    
    // Method to change task status
    fn change_task_status(&mut self, status: TaskStatus) {

        self.status = status;

    }

    // Method to change task name
    fn change_task_name(&mut self, name: String) {

        self.name = name;

    }

}

// Method to get the enum value as a string
impl fmt::Display for TaskStatus {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            TaskStatus::Complete => write!(f, "Complete"),
            TaskStatus::Incomplete => write!(f, "Incomplete")
        }

    }

}

// Main function to run the programs
fn main() {

    let mut is_running: bool = true;

    let mut task_manager = TaskManager {

        tasks: Vec::new()

    };
    
    println!("===============================");
    println!("========== TO DO APP ==========");
    println!("===============================");

    while is_running {
        
        print_options();

        let mut user_option_input: String = String::new();
        
        print!("Enter your option: ");

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut user_option_input)
            .expect("Failed to read line");

        let user_option_input: u32 = user_option_input.trim().parse().expect("Please type a number");

        match user_option_input {
            
            1 => task_manager.view_tasks(),

            2 => task_manager.view_task(),

            3 => task_manager.add_new_task(),

            4 => task_manager.edit_task(),

            5 => task_manager.delete_task(),

            6 => { is_running = false },

            _default => println!("Wrong Option, Try Again")

        }

    }

}

// Function to print options available
fn print_options() {

    println!("\n1. View Tasks");
    println!("2. View Task by Id");
    println!("3. Add a New Task");
    println!("4. Edit a Task");
    println!("5. Delete a Task");
    println!("6. Quit\n");

    // print!("Pick Your Option: ");

}

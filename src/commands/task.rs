mod get;
use std::env::Args;

enum TaskType {
    Get,
    Create,
    Complete,
}

struct Command<'a> {
    command: TaskType,
    name: &'a str,
    description: &'a str,
}

impl Command<'_> {
    async fn run(&self) {
        match self.command {
            TaskType::Get => get::get().await,
            TaskType::Create => println!("task create!"),
            TaskType::Complete => println!("task complete!"),
        }
    }
}

const COMMANDS: [Command; 3] = [
    Command {
        command: TaskType::Get,
        name: "get",
        description: "Get all active tasks",
    },
    Command {
        command: TaskType::Create,
        name: "create",
        description: "Creates a new task from a given string",
    },
    Command {
        command: TaskType::Complete,
        name: "project",
        description: "Complete a task based on an ID",
    },
];

pub async fn run(input: &mut Args, process_name: String) -> () {
    let text: String = match input.nth(0) {
        Some(thing) => thing,
        None => std::process::exit(1),
    };
    let command = match COMMANDS.iter().find(|x| x.name == text) {
        Some(command) => command,
        None => {
            println!("Invalid Subcommand: {}", text);
            println!("Usage: {} task [command]", process_name);
            print_help_message();
            std::process::exit(1)
        }
    };

    command.run().await
}

fn print_help_message() {
    println!("Commands:");
    COMMANDS
        .iter()
        .for_each(|x| println!("\t{}\t{}", x.name, x.description))
}

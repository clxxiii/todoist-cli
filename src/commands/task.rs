use std::env::Args;

enum TaskType {
    Get,
    Create,
    List,
    Complete,
}

struct Command<'a> {
    command: TaskType,
    name: &'a str,
    description: &'a str,
}

impl Command<'_> {
    fn run(&self) {
        match self.command {
            TaskType::Get => println!("task get!"),
            TaskType::Create => println!("task create!"),
            TaskType::List => println!("task list!"),
            TaskType::Complete => println!("task complete!"),
        }
    }
}

const COMMANDS: [Command; 4] = [
    Command {
        command: TaskType::Get,
        name: "get",
        description: "Get a task based on its ID",
    },
    Command {
        command: TaskType::Create,
        name: "create",
        description: "Creates a new task from a given string",
    },
    Command {
        command: TaskType::List,
        name: "list",
        description: "Generate a list of tasks based on a filter",
    },
    Command {
        command: TaskType::Complete,
        name: "project",
        description: "Complete a task based on an ID",
    },
];

pub fn run(input: &mut Args, process_name: String) -> () {
    let text: String = match input.nth(0) {
        Some(thing) => thing,
        None => {
            print_help_message();
            std::process::exit(1)
        }
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
    command.run();
}

fn print_help_message() {
    println!("Commands:");
    COMMANDS
        .iter()
        .for_each(|x| println!("\t{}\t{}", x.name, x.description))
}

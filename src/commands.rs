use std::env::Args;
mod login;
mod task;

enum CommandType {
    Task,
    Config,
    Login,
    Project,
    Help,
}

struct Command<'a> {
    command: CommandType,
    name: &'a str,
    description: &'a str,
}

impl Command<'_> {
    async fn run(&self, input: &mut Args, process_name: String) {
        match self.command {
            CommandType::Task => task::run(input, process_name).await,
            CommandType::Config => println!("config!"),
            CommandType::Login => login::run(input, process_name),
            CommandType::Project => println!("project!"),
            CommandType::Help => print_help_message(),
        }
    }
}

static COMMANDS: [Command; 5] = [
    Command {
        command: CommandType::Task,
        name: "task",
        description: "Create, get, and complete tasks",
    },
    Command {
        command: CommandType::Config,
        name: "config",
        description: "Change settings related to the CLI",
    },
    Command {
        command: CommandType::Login,
        name: "login",
        description: "Login to todoist",
    },
    Command {
        command: CommandType::Project,
        name: "project",
        description: "Create, get, and update tasks",
    },
    Command {
        command: CommandType::Help,
        name: "help",
        description: "Show this message",
    },
];

pub async fn handle(input: &mut Args, process_name: String) -> () {
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
            println!("Invalid Command: {}", text);
            println!("Usage: {} [command]", process_name);
            print_help_message();
            std::process::exit(1)
        }
    };
    command.run(input, process_name).await;
}

fn print_help_message() {
    println!("Commands:");
    COMMANDS
        .iter()
        .for_each(|x| println!("\t{}\t{}", x.name, x.description))
}

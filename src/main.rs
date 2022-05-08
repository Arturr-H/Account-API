use std::io::Write;
use termcolor::{ Color };
use mongodb::{ sync::Client };

mod commands;
mod output_handler;

/*- The connection URI, might want to grab it from .env later -*/
static MONGO_URI: &str = "mongodb://mongo:27017/nodeapp";

/*- Command line interface loop -*/
fn cli_loop(cmd_map:&Vec<commands::CommandStruct>, tag:&mut String, previous_cmd:&mut Vec<Vec<String>>) {

    /*- Command prefix -*/
    print!("{}> ", tag);
    std::io::stdout()
        .flush()
        .unwrap();

    /*- The command that the user inputted -*/
    let mut command = String::from("");

    /*- Get the user input as valid utf-8 -*/
    if let Err (_) = std::io::stdin().read_line(&mut command) {
        output_handler::throw_res(Color::Red, "Please use ASCII characters only.");
    }

    if command.trim() == "" { return; };

    /*- Split the command into a vector of strings -*/
    let mut command_vec:Vec<String> = command
                                        .split_whitespace()
                                        .map(|s| s.to_string())
                                        .collect();

    /*- Tag the terminal/cli input arrow thing -*/
    if command_vec[0] == "tag" {
        let input_tag = &command_vec[1];
        *tag = input_tag.to_string();
        return;
    }else if command_vec[0] == "prev" {
        println!("YES");
        command_vec = previous_cmd.pop().unwrap_or(vec!["".to_string()]).clone();
    }

    previous_cmd.push(command_vec.clone());

    'main:for ( index, cmd ) in cmd_map.iter().enumerate() {
        /*- If the command is the same as the one in the command map -*/
        if cmd._name == &command_vec[0] {
            /*- Call the function with the variables and the parameters -*/
            (cmd._bind)(command_vec[1..].to_vec());
            break 'main;
        }else if index == cmd_map.len() - 1 {
            /*- If the command is not in the command map -*/
            output_handler::throw_res(Color::Rgb(255, 0, 0), 
                format!("Command <{}> was not found", &command_vec[0]).as_str()
            );
            return;
        }
    }
}

/*- Initialize the mongodb client (check if container is running) -*/
fn init_mongo_client() -> bool {

    output_handler::throw_res(Color::Cyan, "Connection to mongo...");

    /*- Check if connection is Ok(()) -*/
    let connection:bool = Client::with_uri_str(MONGO_URI).is_ok();

    /*- Check if the connection is Ok(()) -*/
    if connection {
        output_handler::throw_res(Color::Green, "Success!");
        return true;
    }else { return false; };
}

/*- Start -*/
fn main() {

    /*- If mongo connection failed, return -*/
    if !init_mongo_client() {
        output_handler::throw_res(Color::Red, "CLI failed to connect to mongo. Exiting");
        return;
    }else {
        output_handler::throw_res(Color::Cyan, "Welcome to the Account-API-CLI\nTo get started, type <help>");
    };

    let mut prev:Vec<Vec<String>> = vec![];

    /*- Cli "start" tag (difficult to explain) -*/
    let mut tag = String::from("==");

    /*- The commands that the user can use -*/
    let cmd_map:Vec<commands::CommandStruct> = commands::get_commands();

    /*- Command line interface loop -*/
    loop { cli_loop(&cmd_map, &mut tag, &mut prev); };
}
use std::io::Write;
use termcolor::{ Color };

mod commands;
mod output_handler;

/*- Command line interface loop -*/
fn cli_loop(cmd_map:&Vec<commands::CommandStruct>, tag:&mut String) {

    /*- Command prefix -*/
    print!("{}> ", tag);
    std::io::stdout()
        .flush()
        .unwrap();

    /*- The command that the user inputted -*/
    let mut command = String::new();

    /*- Get the user input -*/
    std::io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    if command.trim() == "" { return; };

    /*- Split the command into a vector of strings -*/
    let command_vec:Vec<String> = command
                                        .split_whitespace()
                                        .map(|s| s.to_string())
                                        .collect();

    /*- Tag the terminal/cli input arrow thing -*/
    if command_vec[0] == "tag" {
        let input_tag = & command_vec[1];
        *tag = input_tag.to_string();
        return;
    }

    for ( index, cmd ) in cmd_map.iter().enumerate() {
        /*- If the command is the same as the one in the command map -*/
        if cmd._name == command_vec[0] {
            /*- Call the function with the variables and the parameters -*/
            (cmd._bind)(command_vec[1..].to_vec());
            break;
        }else if index == cmd_map.len() - 1 {
            /*- If the command is not in the command map -*/
            output_handler::throw_res(Color::Rgb(255, 0, 0), 
                format!("Command <{}> was not found", command_vec[0]).as_str()
            );

            return;
        }
    }
}

/*- Start -*/
fn main() {

    /*- Cli "start" tag (difficult to explain) -*/
    let mut tag = String::from("==");

    /*- The commands that the user can use -*/
    let cmd_map:Vec<commands::CommandStruct> = commands::get_commands();

    /*- Command line interface loop -*/
    loop { cli_loop(&cmd_map, &mut tag); };
}
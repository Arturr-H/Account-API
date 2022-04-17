use termcolor::{ Color };
use std::process::Command;
use crate::output_handler;
use mongodb::{
    bson::doc,
    sync::Client,
};

/*- The connection URI, might want to grab it from .env later -*/
static MONGO_URI: &str = "mongodb://mongo:27017/nodeapp";
fn initialize_client() -> mongodb::sync::Database {
    let client:mongodb::sync::Client = Client::with_uri_str(MONGO_URI).expect("Failed to initialize client");
    return client.database("DockerMongo");
}

/*- All the parameters a user-variable has -*/
#[derive(Debug)]
#[derive(Clone)]
pub struct CommandStruct<'a> {
    pub _name: &'a str,/*- Name is for calling the function via a String -*/
    pub _usage: &'a str, /*- Usage is displayed when <help> command is triggered -*/

    /*--- Bind this to some function --
        first = variables, second = params -*/
    pub _bind: fn(Vec<String>) -> ()
}

/*=------------------=*/
/*=----FUNCTIONS-----=*/
/*=----▼▼▼▼▼▼▼▼▼-----=*/

fn help<P>(_:P) {

    /*- Get all available commands, and display their usage x spaces to the right of the name -*/
    /*- The x is determined by the longest command name -*/
    let mut max_len = 0;

    /*- Find the longest command name -*/
    for cmd in get_commands() {
        if cmd._name.len() > max_len {
            max_len = cmd._name.len();
        }
    }

    /*- Display them -*/
    for cmd in get_commands() {
        print!("{} {}", cmd._name, " ".repeat(max_len - cmd._name.len()));
        println!("| {}", cmd._usage);
    }
}

/*- Reset -*/
fn reset<P>(_:P) {
    output_handler::throw_res(Color::Cyan, "Clearing...");
    std::process::Command::new("clear").status().unwrap();
}

/*- Reset just another name -*/
fn clear<P>(_:P) {
    reset(0);
}

/*- Exit -*/
fn exit<P>(_:P) {
    output_handler::throw_res(Color::Cyan, "Exiting...");
    std::process::exit(0);
}

/*- Read terminal commands -*/
fn cmd(argv:Vec<String>) {

    /*- The command that the user gave -*/
    let command = argv.join(" ");

    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    
    output_handler::throw_res(Color::Green, &String::from_utf8_lossy(&result.stdout));
}

/*- Create a new user -*/
fn create_user(argv:Vec<String>) {

    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection("users");

    /*- A vector of keys and values that the user inputted -*/
    /*- The keys and values are strings like this - key:val-*/
    let mut user_data:Vec<Vec<String>> = Vec::new();

    /*- Loop through the arguments -*/
    for arg in argv {
        /*- Split the argument into key and value -*/
        let split_arg = arg.split(":").collect::<Vec<&str>>();

        /*- Push the key and value into the user_data vector -*/
        user_data.push(split_arg.iter().map(|x| x.to_string()).collect());
    }

    /*- The document that the user wants to add with the keys and values -*/
    let mut doc = doc! { };

    /*- Loop through the user_data vector -*/
    for data in user_data {
        /*- Add the key and value to the document -*/
        doc.insert(&data[0], data[1].parse::<i32>().unwrap());
    }

    /*- Insert the document -*/
    coll.insert_one(doc, None).unwrap();

    output_handler::throw_res(Color::Green, "User created!");
}

/*=----▲▲▲▲▲▲▲▲▲-----=*/
/*=----FUNCTIONS-----=*/
/*=------------------=*/


/*- Return all commands -*/
pub fn get_commands() -> Vec<CommandStruct<'static>> {
    return vec![
        CommandStruct { _name: "help",   _usage: "see all available commands.",                _bind: help   },
        CommandStruct { _name: "reset",  _usage: "clear all output - same as <clear>",         _bind: reset  },
        CommandStruct { _name: "clear",  _usage: "clear all output - same as <reset>",         _bind: clear  },
        CommandStruct { _name: "exit",   _usage: "exit the CLI",                               _bind: exit   },
        CommandStruct { _name: "cmd",    _usage: "'cmd <terminal_command>' - run a command",   _bind: cmd    },
        CommandStruct { _name: "create", _usage: "'create_user' - create a new user",          _bind: create_user },
    ];
}
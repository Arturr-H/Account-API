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
    pub _param_required: bool, /*- Parameter is required or not -*/

    /*--- Bind this to some function --
        first = variables, second = params -*/
    pub _bind: fn(Vec<String>) -> ()
}

/*- Beginning of every function that has some sort of input must use this -*/
fn check_argv(argv: &Vec<String>) -> bool {
    if argv.len() == 0 {
        output_handler::throw_res(Color::Red, "No arguments provided whilst function requires that.");
        return false;
    }
    return true;
}

/*=------------------=*/
/*=----FUNCTIONS-----=*/
/*=----▼▼▼▼▼▼▼▼▼-----=*/

fn help(argv:Vec<String>) {

    output_handler::throw_res(Color::Cyan, "Command parameters are documented like this: command <param> <param2>");
    output_handler::throw_res(Color::Cyan, "Params are separated by spaces, and do not contain the angle brackets.");
    output_handler::throw_res(Color::Cyan, "Sometimes parameters are optional, and are marked with a '?', like this: <param>?");
    output_handler::throw_res(Color::Cyan, "Some functions can have diffrent input parameters. Like the get function:");
    output_handler::throw_res(Color::Cyan, "get ['all', 'where <key> key <val>'] - these params are enclosed in square brackets.");
    output_handler::throw_res(Color::Cyan, "The following commands are available:");

    let mut all_commands:Vec<CommandStruct<'static>> = get_commands();

    /*- If there was a command name specified then we'll output the usage -*/
    if argv.len() > 0 {
        let cmd_name = &argv[0];

        /*- Get all commands-*/
        for command in &all_commands {
            if command._name == cmd_name {
                println!("{}", command._usage);
                return;
            }
        }

        output_handler::throw_res(Color::Red, "Command not found!");
        return;
    }

    /*- Get all available commands, and display their usage x spaces to the right of the name -*/
    /*- The x is determined by the longest command name -*/
    let mut max_len = 0;

    /*- Find the longest command name -*/
    for cmd in &all_commands {
        if cmd._name.len() > max_len {
            max_len = cmd._name.len();
        }
    }

    all_commands.push(CommandStruct { _name: "tag", _usage: "tag <name> - tags the input arrow", _param_required: false, _bind: help });

    /*- Display them -*/
    for cmd in &all_commands {
        print!("| {} | {} {}", 
            match cmd._param_required {
                true => "*",
                false => "x",
            },
            cmd._name,
            " ".repeat(max_len - cmd._name.len())
        );
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

    /*- Validate the input -*/
    if !check_argv(&argv) { return; }

    /*- The command that the user gave -*/
    let command = argv.join(" ");

    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    
    output_handler::throw_res(Color::Green, &String::from_utf8_lossy(&result.stdout));
}

/*- Create a new document -*/
fn create(argv:Vec<String>) {

    /*- Validate the input -*/
    if !check_argv(&argv) { return; }

    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>("users");

    /*- A vector of keys and values that the user inputted -*/
    /*- The keys and values are strings like this - key:val-*/
    let mut input_data:Vec<Vec<String>> = Vec::new();

    /*- Loop through the arguments -*/
    for arg in argv {
        /*- Split the argument into key and value -*/
        let split_arg = arg.split(":").collect::<Vec<&str>>();

        /*- Push the key and value into the input_data vector -*/
        input_data.push(split_arg.iter().map(|x| x.to_string()).collect());
    }

    /*- The document that the user wants to add with the keys and values -*/
    let mut doc = doc! { };

    /*- Loop through the input_data vector -*/
    for data in input_data {
        /*- Add the key and value to the document -*/
        doc.insert(&data[0], &data[1]);
    }

    /*- Insert the document -*/
    coll.insert_one(doc, None).unwrap();

    output_handler::throw_res(Color::Green, "Document created!");
}

/*- Get things from dbs -*/
fn get(argv:Vec<String>) {

    /*- Validate the input -*/
    if !check_argv(&argv) { return; }
    
    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>("users");

    /*- What the user wants to get -*/
    let to_get = &argv[0];

    if to_get == "all" {
        /*- Get all the users -*/
        let documents = match coll.find(None, None) {
            Ok(cursor) => cursor,
            Err(_) => {
                output_handler::throw_res(Color::Red, "Failed to get documents!");
                return;
            }
        };

        /*- Loop through and print every document -*/
        for document in documents.map(|doc| doc.unwrap()) {
            println!("{}", document.to_string());
        }

    }

    /*- This command will look like this: Get where name is artur 
        aka search for a document with the matching k&v:s         -*/
    else if &argv.len() >= &3 && to_get == "where" && &argv[2] == "is" {

        // where=0 key=1 is=2 value=3
        /*- Key -*/
        let k = &argv[1];

        /*- Value -*/
        let v = &argv[3];

        /*- Get the document by the key and value -*/
        let document = coll.find_one(Some(doc! { k:v }), None).unwrap();

        if document.is_none() {
            output_handler::throw_res(Color::Red, "Document not found!");
            return;
        }else {   
            /*- Print the document -*/
            println!("{:?}", document);
        }
    }


    else {
        output_handler::throw_res(Color::Red, "Invalid syntax! Write <help get> for further information.");
    }
}

fn shit<P>(_:P) {
    output_handler::throw_res(Color::Cyan, "Shitting right now...");
}

/*=----▲▲▲▲▲▲▲▲▲-----=*/
/*=----FUNCTIONS-----=*/
/*=------------------=*/


/*- Return all commands -*/
pub fn get_commands() -> Vec<CommandStruct<'static>> {
    return vec![
        CommandStruct { _name: "help",   _usage: "help <command name>?",                       _bind: help,            _param_required: false },
        CommandStruct { _name: "reset",  _usage: "clear all output - same as <clear>",         _bind: reset,           _param_required: false },
        CommandStruct { _name: "clear",  _usage: "clear all output - same as <reset>",         _bind: clear,           _param_required: false },
        CommandStruct { _name: "exit",   _usage: "exit the CLI",                               _bind: exit,            _param_required: false },
        CommandStruct { _name: "cmd",    _usage: "cmd <terminal_command>",                     _bind: cmd,             _param_required: true },
        CommandStruct { _name: "create", _usage: "create <key:val> <some_key:some_val>",       _bind: create,          _param_required: true },
        CommandStruct { _name: "get",    _usage: "get ['all', 'where <key> is <val>']",        _bind: get,             _param_required: true },
        CommandStruct { _name: "shit",   _usage: "only for testing.",                          _bind: shit,            _param_required: false },
    ];
}
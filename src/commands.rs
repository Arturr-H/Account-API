use termcolor::{ Color };
use std::process::Command;
use crate::output_handler;
use mongodb::{
    bson::doc,
    sync::Client,
};
use std::io::stdin;
use rand::prelude::*;
use reqwest::{
    header,
    header::{ HeaderMap },
    blocking::{ Client as HttpClientBLOCKING }
};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    #[allow(deprecated)]
    static ref CURRENT_COLLECTION: Mutex<String> = Mutex::new(String::from("users"));
    #[allow(deprecated)]
    static ref CURRENT_DATABASE: Mutex<String> = Mutex::new(String::from("DockerMongo"));
}

/*- Create random command usesthese default names n stuff -*/
static NAMES: [&str; 25] = ["artur", "bob", "carl", "david", "emily", "frank", "gabriel", "harry", "ian", "james", "kate", "laura", "matt", "natalie", "olivia", "peter", "quinn", "rachel", "sarah", "taylor", "victoria", "wendy", "xavier", "yvonne", "zoey"];
static LAST_NAMES: [&str; 12] = ["smith", "brown", "davis", "wilson", "williams", "bobson", "hoffman", "harrison", "beck", "jones", "jefferson", "doe"];
static PASSWORDS: [&str; 3] = ["password", "12345", "safe"];
fn get_random(list: &Vec<&str>) -> String {
    let index = rand::thread_rng().gen_range(0..list.len());
    return list[index].to_string();
}

/*- IMPORTANT: Green color = output, cyan = status messages like "clearing...", yellow = input -*/
/*- The connection URI, might want to grab it from .env later -*/
static MONGO_URI: &str = "mongodb://mongo:27017/nodeapp";
static BACKEND_URL: &str = "https://wss.artur.red";
fn initialize_client() -> mongodb::sync::Database {
    let client:mongodb::sync::Client = Client::with_uri_str(MONGO_URI).expect("Failed to initialize client");
    return client.database(CURRENT_DATABASE.lock().unwrap().as_str());
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

/*- Confirmation function that will be used for "dangerous functions" -*/
fn confirm(question:&str) -> bool {

    let mut input = String::new();

    /*- Print the question -*/
    output_handler::throw_res(Color::Yellow, format!("{} [y/n]", question).as_str());

    /*- Get the standard input -*/
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    input = input.trim().to_string();

    if input == "y" { return true; };
    return false;
}

/*=------------------=*/
/*=----FUNCTIONS-----=*/
/*=----▼▼▼▼▼▼▼▼▼-----=*/

fn help(argv:Vec<String>) {

    if &argv.len() == &0 {
        output_handler::throw_res(Color::Cyan, "Command parameters are documented like this: command <param> <param2>");
        output_handler::throw_res(Color::Cyan, "Params are separated by spaces, and do not contain the angle brackets.");
        output_handler::throw_res(Color::Cyan, "Sometimes parameters are optional, and are marked with a '?', like this: <param>?");
        output_handler::throw_res(Color::Cyan, "Some functions can have diffrent input parameters. Like the get function:");
        output_handler::throw_res(Color::Cyan, "get ['all', 'where <key> key <val>'] - these params are enclosed in square brackets.");
        output_handler::throw_res(Color::Cyan, "The following commands are available:");
    }

    let mut all_commands:Vec<CommandStruct<'static>> = get_commands();

    /*- If there was a command name specified then we'll output the usage -*/
    if argv.len() > 0 {
        let cmd_name = &argv[0];

        /*- Get all commands-*/
        for command in &all_commands {
            if command._name == cmd_name {
                output_handler::throw_res(Color::Green, command._usage);
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

    /*- This command is special, it. doesn't exist in the command vec -*/
    all_commands.push(CommandStruct { _name: "tag", _usage: "tag <name> - tags the input arrow", _param_required: true, _bind: help });

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
        output_handler::throw_res(Color::Green, cmd._usage);
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

    if &argv[0] == "random" {
        let client:HttpClientBLOCKING = reqwest::blocking::Client::new();

        /*- POST headers -*/
        let mut headers:HeaderMap = HeaderMap::new();

        /*- The required headers are listed in API.js -*/
        headers.insert( header::CONTENT_TYPE, header::HeaderValue::from_static("application/json") );
        headers.insert( "username", get_random(&NAMES.to_vec()).to_string().parse().unwrap() );
        headers.insert( "displayname", get_random(&NAMES.to_vec()).to_string().parse().unwrap() );
        headers.insert( "email", format!("{}@{}.com", get_random(&NAMES.to_vec()), get_random(&LAST_NAMES.to_vec())).parse().unwrap() );
        headers.insert( "password", get_random(&PASSWORDS.to_vec()).to_string().parse().unwrap() );

        /*- Post with name and email headers -*/
        let res = client.post(format!("{}/api/create-account", BACKEND_URL))
            .headers(headers)
            .send()
            .unwrap_or_else(|e| {
                output_handler::throw_res(Color::Red, format!("{}", e).as_str());
                std::process::exit(1);
            });

            println!("shitting");

        /*- Get the response -*/
        let body = res.text().unwrap();

        /*- Check if the response was successful -*/
        println!("{}", body);
        return;
    }

    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>(CURRENT_COLLECTION.lock().unwrap().as_str());

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

/*- Delete documents -*/
fn delete(argv:Vec<String>) {

    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>(CURRENT_COLLECTION.lock().unwrap().as_str());

    let _get_where_is = |key:&String, val:&String| -> mongodb::sync::Cursor<mongodb::bson::Document> {

        /*- Get the document by the key and value -*/
        let documents = coll.find(Some(doc! { key:val }), None).unwrap();

        /*- Print the document -*/
        return documents;
    };

    /*- If the user wants to delete all documents -*/
    if &argv.len() == &1 && &argv[0] == "all" {
        if confirm("Delete all documents?") == true {

            /*- Delete everything -*/
            coll.delete_many(doc! { }, None).unwrap();

            output_handler::throw_res(Color::Green, "All documents deleted!");
        }
    }else if &argv.len() >= &5 {

        /*- Get the key and value -*/
        let key = &argv[2];
        let val = &argv[4];

        /*- Get the documents -*/
        let amount_of_documents = _get_where_is(key, val).count();

        /*- If there are no documents -*/
        if &amount_of_documents == &0 {
            output_handler::throw_res(Color::Red, "No documents found!");
            return;
        }else {
            /*- Delete the documents -*/
            if confirm(format!("Delete {} document(s)?", &amount_of_documents).as_str()) == true {
                coll.delete_many(doc! { key:val }, None).unwrap();
                output_handler::throw_res(Color::Green,
                    format!("{} document(s) deleted!", &amount_of_documents).as_str()
                );
            }
        }
    }
}

/*- Update documents -*/
fn update(argv:Vec<String>) {

    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>(CURRENT_COLLECTION.lock().unwrap().as_str());

    //Object id as first parameter, key and value as second parameter (that we want to update)
    //Find the document by the id and update it
    //Get the document by the input id
    let doc = coll.find_one(Some(doc! { "_id":argv[0].parse::<mongodb::bson::oid::ObjectId>().unwrap() }), None).unwrap();

    /*- If the document is not found -*/
    if doc.is_none() {
        output_handler::throw_res(Color::Red, "Document not found!");
        return;
    }else {
        /*- Get the document -*/
        let mut doc = doc.unwrap();

        /*- Get the key and value -*/
        let kv = argv[1].split(":").collect::<Vec<&str>>();

        /*- First we'll check if the key already exists -*/
        if doc.contains_key(&kv[0]) {
            /*- If the key already exists -*/
            if confirm(format!("The key <{}> ({}) already exists. Do you want to overwrite it?", &kv[0], doc.get(&kv[0]).unwrap()).as_str()) == true {
                /*- Update the document -*/
                doc.insert(kv[0], kv[1]);
            }else {
                return;
            }
        }else {
            /*- If the key doesn't exist -*/
            doc.insert(kv[0], kv[1]);
        }
        
        /*- Update the document -*/
        coll.replace_one(doc! { "_id":argv[0].parse::<mongodb::bson::oid::ObjectId>().unwrap() }, doc, None).unwrap();

        output_handler::throw_res(Color::Green, "Document updated!");
    }
}

/*- Get things from dbs -*/
fn get(argv:Vec<String>) {

    /*- Validate the input -*/
    if !check_argv(&argv) { return }
    
    /*- Get the database and the collection that we are using -*/
    let db:mongodb::sync::Database = initialize_client();

    /*- User collection -*/
    let coll = db.collection::<mongodb::bson::Document>(CURRENT_COLLECTION.lock().unwrap().as_str());

    /*- What the user wants to get -*/
    let to_get = &argv[0];

    //
    //  FUNCTIONS
    //

    /*- The where <k> is <v> function -*/
    let _get_where_is = |key:&String, val:&String| -> mongodb::sync::Cursor<mongodb::bson::Document> {

        /*- Get the document by the key and value -*/
        let documents = coll.find(Some(doc! { key:val }), None).unwrap();

        /*- Print the document -*/
        return documents;
    };
    /*- The get-all-documents-function -*/
    let _get_all = || -> Vec<mongodb::bson::Document> {
        /*- Get all the users -*/
        let documents = match coll.find(None, None) {
            Ok(cursor) => cursor,
            Err(_) => {
                output_handler::throw_res(Color::Red, "Failed to get documents!");
                return vec![];
            }
        };

        return documents.map(|doc| doc.unwrap()).collect();
    };

    //
    //  END FUNCTIONS
    //

    if to_get == "all" && &argv.len() == &1 {
        /*- Get all the users -*/
        let documents = _get_all();

        /*- Loop through and print every document -*/
        for doc in documents {
            output_handler::throw_res(Color::Green, &doc.to_string());
        }
    }

    /*- This command will look like this: Get where name is artur 
        aka search for a document with the matching k&v:s         -*/
    else if &argv.len() >= &5 && &argv[1] == "where" {
        // where=0 key=1 is=2 value=3
        /*- Key -*/
        let k = &argv[2];

        /*- Value -*/
        let v = &argv[4];

        /*- Get the document by the key and value -*/
        let documents = _get_where_is(k, v);
        for doc in documents {
            output_handler::throw_res(Color::Green, &doc.unwrap().to_string());
        }
    }

    /*- Get the length of all documents -*/
    else if to_get == "length" && &argv.len() >= &2 && &argv[1] == "of" {
        let get_of = &argv[2];

        if get_of == "all" && argv.len() == 3 {
            /*- Get all the users -*/
            let documents = coll.count_documents(None, None).unwrap();

            /*- Print the length of all the documents -*/
            output_handler::throw_res(Color::Green, &documents.to_string());
        }else if &argv.len() >= &6 && &argv[3] == "where" && &argv[5] == "is" {
            // -1  0      1  2   3     4    5  6
            // get length of all where name is artur

            /*- Key -*/
            let k = &argv[4];

            /*- Value -*/
            let v = &argv[6];

            /*- Get the document by the key and value -*/
            let documents:Vec<mongodb::bson::Document> = _get_where_is(k, v).map(|doc| doc.unwrap()).collect();

            /*- Print the length of the document -*/
            output_handler::throw_res(Color::Green,
                &documents.len().to_string()
            );
        }else {
            output_handler::throw_res(Color::Red, "Invalid parameters!");
        }
    }
    else {
        output_handler::throw_res(Color::Red, "Invalid syntax! Write <help get> for further information.");
    }
}

/*- Switch collection -*/
fn collection(argv:Vec<String>) {

    /*- Validate the input -*/
    if !check_argv(&argv) { return }

    if &argv.len() > &1 && &argv[0] == "switch" {

        /*- The collection the user wants to work with -*/
        let to_coll = &argv[1];

        /*- Change the CURRENT_COLLECTION -*/
        *CURRENT_COLLECTION.lock().unwrap() = to_coll.to_string();
    }else if &argv[0] == "get" {
        
        /*- Show the user what collection they're in -*/
        output_handler::throw_res(Color::Green, &CURRENT_COLLECTION.lock().unwrap().to_string());
    }
}

/*- Switch database -*/
fn database(argv:Vec<String>) {

    /*- Validate the input -*/
    if !check_argv(&argv) { return }

    if &argv.len() > &1 && &argv[0] == "switch" {

        /*- The database the user wants to work with -*/
        let to_db = &argv[1];

        /*- Change the CURRENT_DATABASE -*/
        *CURRENT_DATABASE.lock().unwrap() = to_db.to_string();

    }else if &argv[0] == "get" {
        
        /*- Show the user what database they're working with -*/
        output_handler::throw_res(Color::Green, &CURRENT_DATABASE.lock().unwrap().to_string());
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
        CommandStruct { _name: "help",       _usage: "help <command name>?",                            _bind: help,            _param_required: false },
        CommandStruct { _name: "reset",      _usage: "clear all output - same as <clear>",              _bind: reset,           _param_required: false },
        CommandStruct { _name: "clear",      _usage: "clear all output - same as <reset>",              _bind: clear,           _param_required: false },
        CommandStruct { _name: "exit",       _usage: "exit the CLI",                                    _bind: exit,            _param_required: false },
        CommandStruct { _name: "cmd",        _usage: "cmd <terminal_command>",                          _bind: cmd,             _param_required: true },
        CommandStruct { _name: "create",     _usage: "create <key:val> <some_key:some_val>",            _bind: create,          _param_required: true },
        CommandStruct { _name: "get",        _usage: "get ['all', 'all where <key> is <val>', 'length of ['all', 'where <key> is <val>']']", _bind: get, _param_required: true },
        CommandStruct { _name: "shit",       _usage: "only for testing.",                               _bind: shit,            _param_required: false },
        CommandStruct { _name: "delete",     _usage: "delete [all, all where <key> is <val>]",          _bind: delete,          _param_required: true },
        CommandStruct { _name: "update",     _usage: "update <objectid> <key:val>",                     _bind: update,          _param_required: true },
        CommandStruct { _name: "collection", _usage: "collection ['switch <collection_name>', 'get']",  _bind: collection,      _param_required: true },
        CommandStruct { _name: "database",   _usage: "database ['switch <database_name>', 'get']",      _bind: database,        _param_required: true },
    ];
}
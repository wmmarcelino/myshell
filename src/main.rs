use std::io::{self, Write};
use std::{borrow::Borrow, env, path, process};

#[derive(Clone)]
enum States {
    Alfanumeric,
    SingleQuote,
    DoubleQuote,
    Escape,
    EnvVar,
}

fn parse_cli (string_cli: String) -> (String, Vec<String>) {
    let mut args = Vec::new();
    let mut current_state: States = States::Alfanumeric;
    let mut previous_state: States = States::Alfanumeric;
    let mut arg = String::new();
    let mut env_var = String::new();
    

    for c in string_cli.trim().chars() {

        match current_state {

            States::Alfanumeric => 
                {
                    match c {
                        '\'' => current_state = States::SingleQuote,

                        '\"' => current_state = States::DoubleQuote,

                        '\\' => 
                            {
                                previous_state = States::Alfanumeric;
                                current_state = States::Escape;
                            },
                        
                        ' ' => 
                            if !arg.is_empty() {
                                args.push(arg.clone());
                                arg.clear();
                            },

                        _ => arg.push(c),
                    }
                },
            States::SingleQuote =>
                {
                    match c {
                        '\'' => current_state = States::Alfanumeric,
                        _ => arg.push(c),
                    }
                },

            States::DoubleQuote => 
                {
                    match c {
                        '"' => current_state = States::Alfanumeric,

                        '\\' =>
                            {
                                previous_state = States::DoubleQuote;
                                current_state = States::Escape;
                            },
                        
                        '$' =>
                            {
                                previous_state = States::DoubleQuote;
                                current_state = States::EnvVar;
                            }
                        _ => arg.push(c),
                    }
                },

            States::EnvVar =>
                {
                    match c {
                        ' ' | '\\' | '\"' => 
                            {
                                let value = env::var(&env_var).unwrap();
                                for v in value.chars() {
                                    arg.push(v);
                                }
                                env_var.clear();
                                if c != '\"' {
                                    current_state = previous_state.clone();
                                } else {
                                    current_state = States::Alfanumeric;
                                }
                            }
                        _ => env_var.push(c),
                    }
                }

            States::Escape =>
                {
                    match previous_state {
                        States::Alfanumeric =>
                            {
                                if c != '\\' && c != '\"' && c != ' ' && c != 'n' {
                                    arg.push('\\');
                                }
                                arg.push(c);
                            },
                        
                        States::DoubleQuote =>
                            {
                                if c != '\\' && c != '\"' {
                                    arg.push('\\');
                                }
                                arg.push(c);
                            },
                        
                        _ => arg.push(c),
                    }
                    current_state = previous_state.clone();
                }
        }
    }

    if !arg.is_empty() {
        args.push(arg.clone());
    }

    (args[0].clone(), args[1..].to_vec())
}


fn main() {
    let stdin = io::stdin();
    let builtin  = ["exit", "echo", "type", "pwd"];
    let path_env = env::var("PATH").unwrap();

    loop {
        let mut input = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();

        let (cmd, args) = parse_cli(input);

        match cmd.borrow() {
            "" => (),
            "exit" => process::exit(args[0].parse::<i32>().unwrap()),
            "echo" => println!("{}", args.join(" ")),
            "type" =>
                if builtin.contains(&args[0].borrow()) {
                    println!("{} is a shell builtin", args[0]);
                }
                else {
                    let mut path_list = path_env.split(":");
                    if let Some(path) = path_list.find(|path| path::Path::new(&format!("{}/{}", path, args[0])).is_file()) {
                        println!("{} is {}/{}", args[0], path, args[0]);
                    }
                    else {
                        println!("{}: not found", args[0]);
                    }
                },
            "pwd" => println!("{}", env::current_dir().unwrap().to_string_lossy()),
            "cd" => 
                {
                    let dir: String;
                    if args.len() == 0 {
                        dir = ".".to_string();
                    }
                    else {
                        match args[0].borrow() {
                            "~" => dir = env::var("HOME").unwrap(),
                            _ => dir = args[0].to_string(),
                        }
                    }

                    let current_dir= env::set_current_dir(path::Path::new(&dir));
                    if current_dir.is_ok() {

                    } else {
                        println!("{}: No such file or directory", dir);
                    }
                },

            _ => 
                {
                    let output = process::Command::new(&cmd).args(args).output();
                    if output.is_err() {
                        println!("{}: command not found", &cmd);
                    } else {
                        print!("{}", String::from_utf8_lossy(&output.unwrap().stdout));
                    }
                }
        }
    }
}
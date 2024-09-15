mod core_all_crack;
mod aux;
mod core_identify;

use std::path::Path;
use aux::read_words_from_file;
use std::time::Instant;

use clap::{Arg, command, Command};
use crate::core_all_crack::all_crack;

use colored::*;

fn main() {

    banner();

    let match_command = command!()
        .subcommand(Command::new("id")
            .about("Identify the hash smartly")
        )
        .subcommand(Command::new("idc")
            .about("Perform identification and brute-force against probable algos only")
        )
        .subcommand(Command::new("allc")
            .about("Brute-force the hash (without id, against all algos)")
        )
        .arg(Arg::new("hash").short('x').long("hash").help("The hash to be identified or cracked"))
        .arg(Arg::new("wordlist").short('w').long("wlist").help("The wordlist to use for brute-forcing"));

    let match_res = match_command.to_owned().get_matches();


    match match_res.subcommand_name() {
        Some("id") | Some("allc") | Some("idc") => {
            if match_res.contains_id("hash") {
                let orig_hash: &String = match_res.get_one::<String>("hash").unwrap();

                match match_res.subcommand_name() {
                    Some("id") => run_id(orig_hash),
                    Some("allc") => {
                        let filepath : &String = match_res.get_one::<String>("wordlist").unwrap();
                        run_allc(filepath, orig_hash);
                    }
                    Some("idc") => {
                        let _filepath : &String = match_res.get_one::<String>("wordlist").unwrap();
                    }
                    _ => {}
                }
            }
            else {
                println!("{}", "ERROR : Hash not entered. Enter a hash using -x <hash>.".red());
                println!();
                return;
            }
        }
        _ => {
            println!("{}", "ERROR : [COMMAND] not specified. Enter a command : id / idc / allc.\n".red());
            match_command.to_owned().print_help().expect("TODO: panic message");
        }
    }

    println!();
}

fn run_id(orig_hash: &String) {
    println!("  ➤   Mode selected : Identification");
    print!("  ➤   Hash inputted : ");
    println!("{}", orig_hash.green());
    println!("  ➤   Attempting to identify the hash against 18 hashes.");
    println!("{}", "      Note: Use --info for more information.".blue());
}

fn run_allc(filepath: &str, orig_hash: &String){

    match read_words_from_file(filepath) {
        Ok(words) => {

            println!("  ➤   Mode selected : Brute-force without identification");
            print!("{}", "  ➤   ");
            println!("{}", "⚠  Note : Identification is off. Brute-forcing may take much longer.".red());
            print!("  ➤   Hash inputted : ");
            println!("{}", orig_hash.green());
            print!("  ➤   Wordlist used : ");
            let file_name = Path::new(filepath).file_name().unwrap().to_str().unwrap();
            println!("{}", file_name.green());
            println!("{}", "      Note: Use --info for more information\n\n".blue());

            let start_t = Instant::now();
            all_crack(words, orig_hash.clone());
            let end_t = Instant::now();

            let millis = (end_t-start_t).as_millis();
            let mins = millis / 60000;
            let secs = (millis % 60000) / 1000;
            let millis = millis % 1000;

            if mins>0 { println!("\nTime taken : {}m {}s {}ms", mins, secs, millis); }
            else if secs > 0 { println!("\nTime taken : {}s {}ms", secs, millis); }
            else { println!("\nTime taken : {}ms", millis); }
        }
        Err(err) => {
            println!("Error reading file: {}", err);
        }
    }
}

fn banner(){
    let banner_str = "\n\n
     /$$                           /$$       /$$       /$$                 /$$
    | $$                          | $$      | $$      | $$                | $$
    | $$$$$$$   /$$$$$$   /$$$$$$$| $$$$$$$ | $$$$$$$ | $$  /$$$$$$   /$$$$$$$  /$$$$$$
    | $$__  $$ |____  $$ /$$_____/| $$__  $$| $$__  $$| $$ |____  $$ /$$__  $$ /$$__  $$
    | $$  \\ $$  /$$$$$$$|  $$$$$$ | $$  \\ $$| $$  \\ $$| $$  /$$$$$$$| $$  | $$| $$$$$$$$
    | $$  | $$ /$$__  $$ \\____  $$| $$  | $$| $$  | $$| $$ /$$__  $$| $$  | $$| $$_____/
    | $$  | $$|  $$$$$$$ /$$$$$$$/| $$  | $$| $$$$$$$/| $$|  $$$$$$$|  $$$$$$$|  $$$$$$$
    |__/  |__/ \\_______/|_______/ |__/  |__/|_______/ |__/ \\_______/ \\_______/ \\_______/\n\n               Hash identifier and cracker written in Rust, by @impl1cate\n";

    println!("{}", banner_str);
}
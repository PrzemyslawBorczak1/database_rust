use clap::Parser;
use std::io::{self, Write};

use database::model::{AnyDatabase, Database};
use database::parsing::SQLParser;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long = "db")]
    db: String,
}

fn main() {
    let cli = Cli::parse();

    let kind = cli.db.trim().to_lowercase();
    let mut db = match kind.as_str() {
        "int" | "i" => AnyDatabase::IntDatabase(Database::new()),
        "string" | "s" => AnyDatabase::StringDatabase(Database::new()),
        other => {
            eprintln!("Invalid --db '{other}'. Use 'string(s)' or 'int(i)'.");
            std::process::exit(1);
        }
    };
    run_query_loop(&mut db);
}

fn run_query_loop(db: &mut AnyDatabase) {
    loop {
        print!("sql> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("/exit") {
            println!("Bye.");
            break;
        }

        if !input.is_empty() {
            execute_query(db, input);
        }
    }
}

fn execute_query(db: &mut AnyDatabase, query: &str) {
    let res = SQLParser::run_query(query, db);
    if let Err(e) = res {
        println!("{e}");
    } else if let Ok(Some(s)) = res {
        println!("{}", s);
    }
}

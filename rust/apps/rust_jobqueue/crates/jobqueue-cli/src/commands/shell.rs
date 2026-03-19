//! Interactive shell command
//!
//! VULNERABILITY: Allows execution of arbitrary SQL via shell

use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;

use jobqueue_db::SqliteStore;

pub async fn run(database: &str) -> Result<()> {
    println!("{}", "JobQueue Interactive Shell".green().bold());
    println!("Type 'help' for commands, 'exit' to quit.\n");

    let store = SqliteStore::new(database)?;

    loop {
        print!("{} ", "jq>".cyan());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" | "q" => {
                println!("Goodbye!");
                break;
            }

            "help" | "?" => {
                print_help();
            }

            "status" => {
                crate::commands::status::run(database, false).await?;
            }

            "queues" => {
                crate::commands::queue::run(database, &crate::QueueCommands::List, false).await?;
            }

            cmd if cmd.starts_with("sql ") => {
                // VULNERABILITY: Arbitrary SQL execution
                let sql = &cmd[4..];
                println!("{}", "Executing SQL...".yellow());

                match store.execute_raw(sql).await {
                    Ok(rows) => println!("Rows affected: {}", rows),
                    Err(e) => println!("{} {}", "Error:".red(), e),
                }
            }

            cmd if cmd.starts_with("search ") => {
                let query = &cmd[7..];
                match store.search_jobs(query, 10).await {
                    Ok(jobs) => {
                        for job in jobs {
                            println!("  {} - {} ({:?})", job.id, job.payload.job_type, job.state);
                        }
                    }
                    Err(e) => println!("{} {}", "Error:".red(), e),
                }
            }

            cmd if cmd.starts_with("get ") => {
                let id = &cmd[4..];
                crate::commands::job::run(
                    database,
                    &crate::JobCommands::Get { id: id.to_string() },
                    false,
                ).await?;
            }

            _ => {
                println!("Unknown command: {}", input.yellow());
                println!("Type 'help' for available commands.");
            }
        }

        println!();
    }

    Ok(())
}

fn print_help() {
    println!("{}", "Available Commands:".green().bold());
    println!();
    println!("  {}  - Show system status", "status".cyan());
    println!("  {}  - List all queues", "queues".cyan());
    println!("  {} - Search for jobs", "search <query>".cyan());
    println!("  {}     - Get job details", "get <id>".cyan());
    println!("  {}    - Execute raw SQL (dangerous!)", "sql <query>".cyan());
    println!("  {}    - Show this help", "help".cyan());
    println!("  {}    - Exit the shell", "exit".cyan());
}

mod commands;
mod payment;
mod powerup;

use std::{sync::Arc, thread, time::Duration};

use anyhow::Context;
use clap::{CommandFactory, Parser};
use commands::Commands;
use payment::{card::Card, payment_context::PaymentContext, providers::Provider};
use powerup::{Powerup, PowerupManager};

#[derive(Parser)]
#[command(version, about = "A test project", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn get_context(provider: &Provider) -> PaymentContext {
    match provider {
        Provider::Card => PaymentContext::new(Box::new(Card {
            number: "3211-4321-5415-6156".to_string(),
        })),
    }
}

fn start_events(manager: Arc<PowerupManager>) {
    thread::spawn(move || {
        let events = vec!["SpeedBoost", "Cube"];
        for event in events {
            thread::sleep(Duration::from_secs_f32(0.5));
            println!("Got event {}", event);
            manager.activate_powerup(event);
        }
    })
    .join()
    .unwrap();
}

fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Some(Commands::Pay { provider }) => {
            let parsed_provider = Provider::from_str(provider)
                .context("There was a problem parsing the payment provider")?;
            let payment_context = get_context(&parsed_provider);
            payment_context.process_payment();
        }
        Some(Commands::Concurrency {}) => {
            let manager = Arc::new(PowerupManager::new());
            start_events(manager.clone());
            println!(
                "Completed. Powerups activated: {:?}",
                manager.get_powerups()
            );

            match manager.powerups_locked() {
                true => {
                    println!("Muted unlocked");
                }
                _ => println!("Mutex locked"),
            };
        }
        None => Args::command().print_help().unwrap(),
    }

    Ok(())
}

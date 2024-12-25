mod commands;
mod payment;
mod powerup;
mod services;
mod middleware;

use std::{sync::Arc, thread, time::Duration};

use actix_web::{App, HttpServer};

use anyhow::Context;

use clap::{CommandFactory, Parser};
use commands::Commands;
use payment::{card::Card, payment_context::PaymentContext, providers::Provider};
use powerup::{Powerup, PowerupManager};
use services::hello;

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
            log::info!("Got event {}", event);
            manager.activate_powerup(event);
        }
    })
    .join()
    .unwrap();
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let cli = Args::parse();

    let log_env = env_logger::Env::default().default_filter_or("info");

    env_logger::Builder::from_env(log_env).format_module_path(false).format_timestamp(None).init();

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
            log::info!(
                "Completed. Powerups activated: {:?}",
                manager.get_powerups()
            );

            match manager.powerups_locked() {
                true => {
                    log::info!("Mutex unlocked")
                }
                _ => log::warn!("Mutex locked"),
            };
        }
        Some(Commands::Serve {}) => {
            log::info!("Starting HTTP Server");
            let _ = HttpServer::new(|| App::new().service(hello))
                .bind(("127.0.0.1", 3000))?
                .run()
                .await;
        }
        None => Args::command().print_help().unwrap(),
    }

    Ok(())
}

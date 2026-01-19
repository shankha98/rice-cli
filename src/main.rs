use console::{Emoji, style};
use dialoguer::{Confirm, Input, Password, theme::ColorfulTheme};
use reqwest::Client;
use std::fs;
use std::io::Write;
use std::path::Path;

static RICE: Emoji<'_, '_> = Emoji("🍚  ", "");
static CHECK: Emoji<'_, '_> = Emoji("✔  ", "");
static CROSS: Emoji<'_, '_> = Emoji("✖  ", "");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        RICE,
        style("Welcome to the Rice CLI Setup").bold().green()
    );
    println!("This utility will walk you through setting up Rice in your project.\n");

    let theme = ColorfulTheme::default();

    // 1. Configuration Questions
    let enable_storage = Confirm::with_theme(&theme)
        .with_prompt("Enable Rice Storage?")
        .default(true)
        .interact()?;

    let enable_state = Confirm::with_theme(&theme)
        .with_prompt("Enable Rice State (AI Agent Memory)?")
        .default(true)
        .interact()?;

    if !enable_storage && !enable_state {
        println!("{}", style("You must enable at least one service.").red());
        return Ok(());
    }

    // Storage Config
    let mut storage_url = String::from("localhost:50051");
    let mut storage_user = String::from("admin");
    let mut storage_token = String::new();
    let mut storage_http_port = String::from("3000");

    if enable_storage {
        println!("\n{}", style("Storage Configuration").bold());

        storage_url = Input::with_theme(&theme)
            .with_prompt("Storage Instance URL")
            .default("localhost:50051".into())
            .interact_text()?;

        storage_user = Input::with_theme(&theme)
            .with_prompt("Storage User")
            .default("admin".into())
            .interact_text()?;

        storage_token = Password::with_theme(&theme)
            .with_prompt("Storage Auth Token/Password")
            .allow_empty_password(true)
            .interact()?;

        storage_http_port = Input::with_theme(&theme)
            .with_prompt("Storage HTTP Port (for verification)")
            .default("3000".into())
            .interact_text()?;
    }

    // State Config
    let mut state_url = String::from("localhost:50051");
    let mut state_token = String::new();
    let mut state_run_id = String::from("default");

    if enable_state {
        println!("\n{}", style("State Configuration").bold());

        state_url = Input::with_theme(&theme)
            .with_prompt("State Instance URL")
            .default("localhost:50051".into())
            .interact_text()?;

        state_token = Password::with_theme(&theme)
            .with_prompt("State Auth Token")
            .allow_empty_password(true)
            .interact()?;

        state_run_id = Input::with_theme(&theme)
            .with_prompt("State Run ID")
            .default("default".into())
            .interact_text()?;
    }

    // 2. Generate rice.config.js
    println!("\n{}", style("Generating configuration files...").bold());

    let config_content = format!(
        "/** @type {{import('rice-node-sdk').RiceConfig}} */\nmodule.exports = {{\n  storage: {{\n    enabled: {},\n  }},\n  state: {{\n    enabled: {},\n  }},\n}};",
        enable_storage, enable_state
    );

    let config_path = Path::new("rice.config.js");
    if config_path.exists() {
        let overwrite = Confirm::with_theme(&theme)
            .with_prompt("rice.config.js already exists. Overwrite?")
            .default(false)
            .interact()?;

        if overwrite {
            fs::write(config_path, config_content)?;
            println!("{} Created rice.config.js", CHECK);
        } else {
            println!("{} Skipped rice.config.js", CHECK);
        }
    } else {
        fs::write(config_path, config_content)?;
        println!("{} Created rice.config.js", CHECK);
    }

    // 3. Update .env
    let env_content = format!(
        "\n# Rice Configuration\nSTORAGE_INSTANCE_URL={}\nSTORAGE_USER={}\nSTORAGE_AUTH_TOKEN={}\nSTORAGE_HTTP_PORT={}\nSTATE_INSTANCE_URL={}\nSTATE_AUTH_TOKEN={}\nSTATE_RUN_ID={}\n",
        storage_url,
        storage_user,
        storage_token,
        storage_http_port,
        state_url,
        state_token,
        state_run_id
    );

    let env_path = Path::new(".env");
    if env_path.exists() {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(env_path)?;
        write!(file, "{}", env_content)?;
        println!("{} Appended to .env", CHECK);
    } else {
        fs::write(env_path, env_content)?;
        println!("{} Created .env", CHECK);
    }

    // 4. Verify Connection
    if enable_storage {
        println!("\n{}", style("Verifying connection to Storage...").bold());

        // Construct HTTP URL from storage_url host and storage_http_port
        let host = if storage_url.contains(":") {
            storage_url.split(':').next().unwrap_or("localhost")
        } else {
            &storage_url
        };

        let health_url = format!("http://{}:{}/health", host, storage_http_port);

        let client = Client::new();
        match client.get(&health_url).send().await {
            Ok(res) => {
                if res.status().is_success() {
                    println!(
                        "{} Successfully connected to Rice Storage at {}",
                        CHECK, health_url
                    );
                } else {
                    println!("{} Connection failed: Status {}", CROSS, res.status());
                    println!("   Please check if your Rice instance is running.");
                }
            }
            Err(e) => {
                println!("{} Connection failed: {}", CROSS, e);
                println!(
                    "   Could not reach {}. Please ensure Rice is running and HTTP port is correct.",
                    health_url
                );
            }
        }
    }

    println!("\n{}", style("Setup complete!").bold().green());
    println!("You can now install the SDK using: npm install rice-node-sdk");

    Ok(())
}

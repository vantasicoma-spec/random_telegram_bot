mod commands;

use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;

use commands::{cmd_coin, cmd_help, cmd_roll, ChatSettingsStore};

/// Имя бота для обработки команд с суффиксом (например, /roll@randoms_roll_bot)
const BOT_USERNAME: &str = "randoms_roll_bot";

#[tokio::main]
async fn main() {
    // Загрузка .env файла из секретов
    // Порядок поиска:
    // 1. /etc/secrets/.env (Unix/Linux)
    // 2. ./secrets/.env (относительно корня проекта)
    // 3. .env (текущая директория)
    let secrets_path_unix = std::path::Path::new("/etc/secrets/.env");

    // Ищем secrets/.env начиная от текущей директории и поднимаясь вверх
    let secrets_path_relative = find_secrets_file();

    log::info!("Looking for secrets at: {:?}", secrets_path_relative);

    if secrets_path_unix.exists() {
        dotenvy::from_path(secrets_path_unix).ok();
        log::info!("Loaded secrets from /etc/secrets/.env");
    } else if secrets_path_relative.exists() {
        dotenvy::from_path(&secrets_path_relative).ok();
        log::info!("Loaded secrets from {:?}", secrets_path_relative);
    } else {
        dotenvy::dotenv().ok();
        log::info!("Loaded .env from current directory");
    }

    // Устанавливаем уровень логирования из переменной окружения или по умолчанию info
    std::env::set_var("RUST_LOG", std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    pretty_env_logger::init();
    log::info!("Starting RandomRollBot...");
    log::info!("Working directory: {:?}", std::env::current_dir().ok());
    log::info!("Log level: {}", std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));

    // Получение токена из переменной окружения
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN must be set in secrets file or environment");

    // Создание бота
    let bot = Bot::new(token);

    // Проверка токена
    log::info!("Getting bot info...");
    match bot.get_me().send().await {
        Ok(me) => {
            log::info!("Bot started: @{}", me.user.username.unwrap_or_default());
        }
        Err(e) => {
            log::error!("Failed to get bot info: {:?}", e);
            eprintln!("Error: Failed to connect to Telegram. Check your internet connection.");
            eprintln!("Details: {:?}", e);
            return;
        }
    }

    // Удаляем вебхук если установлен (чтобы не конфликтовал с long polling)
    bot.delete_webhook().send().await.ok();
    log::info!("Webhook deleted (if any), starting long polling...");

    // Установка списка команд для меню
    if let Err(e) = setup_bot_commands(&bot).await {
        log::error!("Failed to set bot commands: {:?}", e);
    }

    let settings: ChatSettingsStore = Arc::new(tokio::sync::RwLock::new(HashMap::new()));

    log::info!("Starting with long polling mode");
    start_with_polling(bot, settings).await;
}

async fn start_with_polling(bot: Bot, settings: ChatSettingsStore) {
    use teloxide::dispatching::Dispatcher;
    
    let bot_for_handler = bot.clone();
    let schema = teloxide::types::Update::filter_message()
        .endpoint(move |msg: teloxide::types::Message| {
            let bot = bot_for_handler.clone();
            let settings = settings.clone();
            async move {
                handle_message(msg, bot, settings).await
            }
        });

    Dispatcher::builder(bot, schema)
        .build()
        .dispatch()
        .await;
}

async fn handle_message(
    msg: Message,
    bot: Bot,
    settings: ChatSettingsStore,
) -> Result<(), teloxide::RequestError> {
    let text = msg.text().unwrap_or("");

    let first_arg = text.split_whitespace().next().unwrap_or("");
    let command = if let Some(at_pos) = first_arg.find('@') {
        let suffix = &first_arg[at_pos + 1..];
        if suffix == BOT_USERNAME {
            &first_arg[..at_pos]
        } else {
            first_arg
        }
    } else {
        first_arg
    };

    let args: Vec<&str> = text.split_whitespace().collect();
    let args_without_command = &args[1..];

    log::debug!("Received message: '{}' from chat {}", text, msg.chat.id);
    log::debug!("Command: '{}', Args: {:?}", command, args_without_command);

    if args.is_empty() {
        return Ok(());
    }

    match command {
        "/help" => {
            log::info!("Handling /help command");
            cmd_help(bot, msg).await
        }
        "/roll" => {
            log::info!("Handling /roll command with args: {:?}", args_without_command);
            cmd_roll(bot, msg, settings).await
        }
        "/coin" => {
            log::info!("Handling /coin command");
            cmd_coin(bot, msg).await
        }
        _ => Ok(()),
    }
}

async fn setup_bot_commands(bot: &Bot) -> Result<(), teloxide::RequestError> {
    use teloxide::types::BotCommand;
    
    let commands = vec![
        BotCommand::new("help", "Справка по боту"),
        BotCommand::new("roll", "Случайное число или <start> <end> диапазон"),
        BotCommand::new("coin", "Бросок монетки"),
    ];
    
    bot.set_my_commands(commands).send().await?;
    log::info!("Bot commands set successfully");
    Ok(())
}

/// Ищет файл secrets/.env, поднимаясь вверх по директориям
fn find_secrets_file() -> std::path::PathBuf {
    let mut current = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    
    // Поднимаемся вверх до 5 уровней
    for _ in 0..5 {
        let secrets = current.join("secrets/.env");
        if secrets.exists() {
            return secrets;
        }
        if !current.pop() {
            break;
        }
    }
    
    std::path::PathBuf::from("secrets/.env")
}

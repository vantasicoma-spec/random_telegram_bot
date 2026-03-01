use std::collections::HashMap;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::Message;

pub type ChatSettingsStore = Arc<tokio::sync::RwLock<HashMap<i64, ChatSettings>>>;

#[derive(Clone, Default)]
pub struct ChatSettings {
    pub default_min: i32,
    pub default_max: i32,
}

pub async fn cmd_roll(
    bot: Bot,
    msg: Message,
    settings: ChatSettingsStore,
) -> Result<(), teloxide::RequestError> {
    let chat_id = msg.chat.id;
    let text = msg.text().unwrap_or("").to_string();

    let args: Vec<&str> = text.split_whitespace().collect();

    let chat_settings = settings.read().await;
    let default_settings = chat_settings
        .get(&chat_id.0)
        .cloned()
        .unwrap_or_else(|| ChatSettings {
            default_min: 1,
            default_max: 100,
        });
    drop(chat_settings);

    if args.len() == 1 {
        // Используем дефолтные настройки
        let result = rand::random_range(default_settings.default_min..=default_settings.default_max);
        bot.send_message(
            chat_id,
            format!(
                "🎲 Выпало число: *{}* \\(диапазон: {} до {}\\)",
                escape_markdown_v2(&result.to_string()),
                escape_markdown_v2(&default_settings.default_min.to_string()),
                escape_markdown_v2(&default_settings.default_max.to_string())
            ),
        )
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
        return Ok(());
    }

    if args.len() == 3 {
        let start: i32 = match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                bot.send_message(chat_id, "Был введён неверный диапазон").await?;
                return Ok(());
            }
        };
        let end: i32 = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                bot.send_message(chat_id, "Был введён неверный диапазон").await?;
                return Ok(());
            }
        };

        if start >= end {
            bot.send_message(chat_id, "Был введён неверный диапазон").await?;
            return Ok(());
        }

        let result = rand::random_range(start..=end);
        bot.send_message(
            chat_id,
            format!("🎲 Выпало число: *{}* \\(диапазон: {} до {}\\)",
                escape_markdown_v2(&result.to_string()),
                escape_markdown_v2(&start.to_string()),
                escape_markdown_v2(&end.to_string())
            ),
        )
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
        return Ok(());
    }

    bot.send_message(chat_id, "Был введён неверный диапазон").await?;
    Ok(())
}

/// Экранирует специальные символы MarkdownV2
fn escape_markdown_v2(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        match c {
            '_' | '*' | '[' | ']' | '(' | ')' | '~' | '`' | '>' | '#' | '+' | '-' | '=' | '|' | '{' | '}' | '.' | '!' => {
                result.push('\\');
                result.push(c);
            }
            _ => result.push(c),
        }
    }
    result
}

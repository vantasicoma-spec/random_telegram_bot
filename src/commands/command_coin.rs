use teloxide::prelude::*;
use teloxide::types::Message;

// Константы для вероятностей (диапазон 0-199999 = 200000 вариантов)
// Орёл: 0-99949 (99950 вариантов = 49.975%)
// Решка: 99950-199899 (99950 вариантов = 49.975%)
// Ребро: 199900-199999 (100 вариантов = 0.05%)
const COIN_TOTAL: u32 = 200_000;
const COIN_HEADS_THRESHOLD: u32 = 99_950;
const COIN_TAILS_THRESHOLD: u32 = 199_900;

pub async fn cmd_coin(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    let roll = rand::random_range(0..COIN_TOTAL);

    let result = if roll < COIN_HEADS_THRESHOLD {
        "🪙 *Орёл\\!*"
    } else if roll < COIN_TAILS_THRESHOLD {
        "🪙 *Решка\\!*"
    } else {
        "🪙 *РЕБРО\\!* \\(Шанс: 0\\.05%\\)"
    };

    bot.send_message(msg.chat.id, result)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

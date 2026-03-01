use teloxide::prelude::*;
use teloxide::types::Message;

pub async fn cmd_help(bot: Bot, msg: Message) -> Result<(), teloxide::RequestError> {
    let help_text = r#"*RandomRollBot \- Бот для генерации случайных чисел*

*Команды:*

/roll \- Случайное число от 1 до 100 или \<start\> \<end\> диапазон

/coin \- Бросок монетки \(Орёл \~49\.975\%, Решка \~49\.975\%, Ребро \~0\.05\%\)

*Примеры:*
/roll 1 10 \- число от 1 до 10✅
/roll 0 200 \- число от 0 до 200✅
/roll \-1 10 \- число от \-1 до 10✅
/roll 22 1 \- неверный диапазон 22 больше 1❌"#;

    bot.send_message(msg.chat.id, help_text)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

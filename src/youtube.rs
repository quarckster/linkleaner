use crate::{message::BotExt, utils::scrub_urls};
use once_cell::sync::Lazy;
use regex::Regex;
use std::error::Error;
use teloxide::{prelude::Requester, types::Message, utils::html::link, Bot};

pub static MATCH_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new("https://(?:www.)?youtube.com/(?P<shorts>shorts/)[A-Za-z0-9-_]{11}.*").unwrap()
});

pub async fn handler(
    bot: Bot,
    message: Message,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    if let Some(text) = scrub_urls(&message)
        && let Some(user) = message.from()
        && let Some(caps) = MATCH_REGEX.captures(&text)
    {
        let text = text.replace(&caps["shorts"], "watch?v=");
        let text = format!("{}: {}", link(user.url().as_str(), &user.full_name()), text);
        let _del = bot.delete_message(message.chat.id, message.id).await;
        bot.try_reply(message, text).await?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::MATCH_REGEX;

    #[test]
    fn verify_regex() {
        let items = vec![
            "https://www.youtube.com/shorts/SqjNixegPKk",
            "https://www.youtube.com/shorts/SqjNixegPKk?feature=share",
            "https://youtube.com/shorts/SqjNixegPKk",
            "https://youtube.com/shorts/JY55-UBtlf8?feature=share",
            "https://youtube.com/shorts/afHFjnPy_vk?feature=share",
        ];
        for item in items {
            assert!(MATCH_REGEX.is_match(item), "{item} failed to match");
            assert!(
                MATCH_REGEX.is_match(&format!("Some leading text {item}")),
                "{item} failed to match"
            );
        }
        assert!(!MATCH_REGEX.is_match("https://youtube.com/watch?v=SqjNixegPKk"));
    }
}

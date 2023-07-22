use teloxide::{
    payloads, prelude::*, requests::JsonRequest, types::Recipient, utils::command::BotCommands,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported: "
)]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "show a chat id")]
    ShowChatId,
    #[command(description = "show message id")]
    ShowMessageId,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    dbg!("Message is {}", &msg);
    let text: String;
    match cmd {
        Command::Help => {
            text = Command::descriptions().to_string();
        }
        Command::ShowChatId => {
            text = format!("This chat id is {}", msg.chat.id);
        }
        Command::ShowMessageId => {
            text = format!("This chat id is {}", msg.chat.id);
        }
    }

    if let Some(thread_id) = msg.thread_id {
        bot.send_message_to_thread(msg.chat.id, thread_id, text)
            .await?;
    } else {
        bot.send_message(msg.chat.id, text).await?;
    }

    Ok(())
}

trait SendToThread: Requester {
    fn send_message_to_thread<C, T>(
        &self,
        chat_id: C,
        thread_id: i32,
        text: T,
    ) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>;
}

impl SendToThread for Bot {
    fn send_message_to_thread<C, T>(&self, chat_id: C, thread_id: i32, text: T) -> Self::SendMessage
    where
        C: Into<Recipient>,
        T: Into<String>,
    {
        Self::SendMessage::new(
            self.clone(),
            payloads::SendMessage::new(chat_id, text).message_thread_id(thread_id),
        )
    }
}

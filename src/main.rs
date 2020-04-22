use rand::prelude::*;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use teloxide::prelude::*;

#[derive(Deserialize)]
struct Config {
    token: String,
    react_rate: u32,
    react_emoticons: Vec<String>,
    default_emoticon: String,
    other_emoticon: String,
    prpr_back: Vec<String>,
}

struct Kotomei {
    bot: Arc<Bot>,
    config: Config,
}

impl Kotomei {
    fn new(config: Config) -> Arc<Self> {
        let bot = Bot::new(&config.token);
        let kotomei = Self { bot, config };
        Arc::new(kotomei)
    }

    async fn run(self: Arc<Self>) {
        Dispatcher::new(self.bot.clone())
            .messages_handler(|rx: DispatcherHandlerRx<Message>| {
                rx.text_messages()
                    .for_each_concurrent(None, move |(ctx, text)| {
                        let ktm = self.clone();
                        async move {
                            ktm.handle_text(ctx, text).await;
                        }
                    })
            })
            .dispatch()
            .await;
    }

    async fn handle_text(&self, ctx: DispatcherHandlerCx<Message>, text: String) {
        match self.get_prpr_target(&text) {
            Some("kotomei") | Some("kotomei_bot") => self.handle_prpr(ctx).await,
            Some(_) => self.handle_prpr_other(ctx).await,
            _ => {}
        }
    }

    async fn handle_prpr(&self, ctx: DispatcherHandlerCx<Message>) {
        let emoticon = {
            let mut rng = thread_rng();
            if rng.gen_ratio(self.config.react_rate, 100) {
                self.config.react_emoticons.choose(&mut rng).unwrap()
            } else {
                &self.config.default_emoticon
            }
        };

        let reply = match self.get_username(&ctx.update) {
            Some(name) if self.config.prpr_back.contains(&name.to_ascii_lowercase()) => {
                format!("/prpr@{} {}", name, emoticon)
            }
            _ => emoticon.to_string(),
        };

        ctx.reply_to(reply).send().await.log_on_error().await;
    }

    async fn handle_prpr_other(&self, ctx: DispatcherHandlerCx<Message>) {
        let reply = &self.config.other_emoticon;
        ctx.reply_to(reply).send().await.log_on_error().await;
    }

    fn get_prpr_target<'a>(&self, text: &'a str) -> Option<&'a str> {
        let mut words = text.split_whitespace();
        let mut pieces = words.next()?.split('@');

        let command = pieces.next();
        let target = pieces.next();

        match command {
            Some("/prpr") => target.or(Some("kotomei_bot")),
            _ => None,
        }
    }

    fn get_username<'a>(&self, message: &'a Message) -> Option<&'a str> {
        message.from()?.username.as_deref()
    }
}

fn read_config() -> Config {
    let mut file = File::open("Kotomei.toml").expect("配置文件 Kotomei.toml 不存在");
    let mut buf = String::new();

    file.read_to_string(&mut buf)
        .expect("配置文件 Kotomei.toml 读取失败");

    let mut config: Config = toml::from_str(&buf).expect("配置文件 Kotomei.toml 解析失败");

    for name in &mut config.prpr_back {
        name.make_ascii_lowercase();
    }

    config
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();

    let config = read_config();
    let kotomei = Kotomei::new(config);

    kotomei.run().await;
}

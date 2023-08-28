use discord_flows::{http::HttpBuilder, Bot, EventModel, ProvidedBot};
use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use notion_flows::notion::models::{Page, PageCreateRequest};
use notion_wasi::{
    ids::{DatabaseId, PageId},
    models::{
        block::{CreateBlock, TextAndChildren},
        properties::PropertyValue::{self, Title},
        // search::PropertyCondition::RichText,
        text::{RichText, RichTextCommon, Text, TextColor},
        Parent,
        Properties,
    },
    NotionApi,
};

use serde::Deserialize;
use serde_json::json;
use slack_flows::send_message_to_channel;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    logger::init();
    let discord_token = env::var("discord_token").unwrap();

    let bot = ProvidedBot::new(discord_token);
    bot.listen(|em| handle(&bot, em)).await;
    // listen_to_event(database, |page| async { handler(page, send).await }).await;
}

async fn handle<B: Bot>(bot: &B, em: EventModel) {
    match em {
        EventModel::Message(msg) => {
            let client = bot.get_client();
            let channel_id = msg.channel_id;
            let content = msg.content;

            if msg.author.bot {
                return;
            }

            _ = client
                .send_message(channel_id.into(), &json!({"content": content}))
                .await;
            // let database = env::var("database").unwrap();
            let notion_token = env::var("notion_token").unwrap();

            let api = NotionApi::new(notion_token).unwrap();

            let page_id = PageId::from_str("21de6521-838e-4003-a964-ca10ec0d9d82").unwrap();

            let database_id = "1c945dd5-c2d5-48de-bb39-2569fdeca4c3";

            // let page = json!({
            //     "parent": {
            //         "database_id": database_id
            //     },
            //     "properties": {
            //         "Name": {
            //             "title": [
            //                 {
            //                     "text": {
            //                         "content": "Tuscan Kale"
            //                     }
            //                 }
            //             ]
            //         },
            //         "Description": {
            //             "rich_text": [
            //                 {
            //                     "text": {
            //                         "content": "A green leafy vegetable."
            //                     }
            //                 }
            //             ]
            //         }
            //     }
            // });

            let parent = Parent::Database {
                database_id: DatabaseId::from_str(database_id).unwrap(),
            };

            let title_block = CreateBlock::Paragraph {
                paragraph: TextAndChildren {
                    rich_text: vec![RichText::Text {
                        rich_text: RichTextCommon {
                            plain_text: "Tuscan Kale".into(),
                            href: None,
                            annotations: None,
                        },
                        text: Text {
                            content: "wow".to_string(),
                            link: None,
                        },
                    }],
                    children: None,
                    color: TextColor::Default,
                },
            };

            let page_request = PageCreateRequest {
                parent,
                properties: Properties {
                    properties: HashMap::new(), // no properties
                },
                children: Some(vec![title_block]),
            };
            match api.create_page(page_request).await {
                Ok(page) => {
                    let content = page.properties.title().unwrap_or("no title".into());
                    send_message_to_channel("ik8", "ch_err", content).await;
                    _ = client
                        .send_message(channel_id.into(), &json!({"content": page.title()}))
                        .await
                }
                Err(_e) => log::error!("Error: {}", _e),
            };

            // match api.get_page(page_id).await {
            //     Ok(page) => {
            //         let content = page.properties.title().unwrap_or("no title".into());
            //         send_message_to_channel("ik8", "ch_err", content).await;
            //         _ = client
            //             .send_message(channel_id.into(), &json!({"content": page.title()}))
            //             .await
            //     }
            //     Err(_e) => log::error!("Error: {}", _e),
            // };
        }
        _ => {}
    }
}

// async fn handler<F>(page: Page, send: F)
// where
//     F: Fn(String),
// {
//     let title = page.title().unwrap_or("<untitled>".to_string());
//     let pros: String = page
//         .properties
//         .properties
//         .iter()
//         .map(|(k, v)| format!("- {k}: {v:?}"))
//         .collect();

// let page = get_page

//     }

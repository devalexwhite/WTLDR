use std::env;

use html2text::from_read;
use openai_api_rust::{
    chat::{ChatApi, ChatBody},
    Auth, Message, OpenAI, Role,
};
use reqwest::blocking;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = &args[1].clone();
    let html = fetch_html(url);

    if html.is_err() {
        panic!("Failed to fetch html");
    }

    let html = html.unwrap();
    let text = html_to_text(&html);

    let summary = summarize_text(&text);
    println!("{}", summary);
}

fn fetch_html(url: &String) -> Result<String, reqwest::Error> {
    blocking::get(url).unwrap().text()
}

fn html_to_text(html: &String) -> String {
    from_read(html.as_bytes(), 20)
}

fn summarize_text(text: &String) -> String {
    let auth = Auth::from_env().unwrap();
    let api_url = "https://api.openai.com/v1/";

    let openai = OpenAI::new(auth, api_url);

    let system_role = Message { role: Role::System, content: String::from("Your job is to summarize text scraped from websites. You will provide a summary of the text in 1 paragraph, followed by 1-5 of the most important bullet points from the text. You will not provide any additional context or describe your output.") };
    let user_role = Message {
        role: Role::User,
        content: format!("Summarize the following text: {}", text),
    };

    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(500),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![user_role, system_role],
    };

    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();

    message.content.clone()
}

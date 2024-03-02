# WTLDR (Web Too Long Didn't Read)

WTLDR is a summarizer for web content that leverages OpenAI. Simply run this CLI command with a URL as the parameter and the application will give a paragraph summary and the top bullet points from the website's text.

## Usage

You must specify your OPENAI API key in the current shell environment. Simply run the following or add to your .zshrc.

`export OPENAI_API_KEY=<YOUR KEY HERE>`

You may also need to give execute permission to the binary, you can do this with the following.

`sudo chmod +x wtldr`

Finally, run the binary with a URL to summarize the web content.

`./wtldr https://www.npr.org/2024/03/01/1235159084/elon-musk-openai-suit-chatgpt-sam-altman-greg-brockman`

## Developing

WTLDR is a simple Rust application built using reqwest to fetch HTML, html2text for extracting text out of the HTML, and openai_api_rust to communicate with GPT3.5.

To develop locally, make sure Rust is installed on your computer, clone the repo, and run `cargo run` in the directory to start the app.

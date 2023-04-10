# Discord Emotion Tracker
Discord Emotion Tracker is an application that collects messages from customer service channels on Discord and uses natural language processing (NLP) techniques for sentiment analysis. By processing these messages, the application can provide valuable insights into customer emotions and feedback.

## Features
- Collect messages from Discord channels
- Send collected messages to a web server for further processing
- Perform sentiment analysis using NLP machine learning models
- Store processed messages and sentiment analysis results in MongoDB
- Built using Rust, Serenity, Axum, and MongoDB

## Architecture Diagram

## Installation and Setup
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/discord-emotion-tracker.git
    ```
2. Change directory to the project folder:
    ```bash
    cd discord-emotion-tracker
    ```
3. Create a config.yaml file in the project root with the following content:
yaml
    ```yaml
    discord_token: "your_discord_token_here"
    mongo_uri: "mongodb://localhost:27017"
    ```
4. Update the `config.yaml` file with your Discord token and MongoDB URI.

5. Build and run the project:

    ```bash
    cargo build --release
    cargo run --release
    ```

## Usage
Once the application is running, it will listen to messages from the configured Discord channels. Messages will be sent to the web server for processing and sentiment analysis. The processed messages and sentiment analysis results will be stored in the MongoDB database.

## Contributing
We welcome contributions! If you'd like to help improve Discord Emotion Tracker, please follow these steps:

1. Fork the repository
2. Create a new branch with your changes
3. Submit a pull request to the main branch

## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Acknowledgements
- [Rust](https://www.rust-lang.org/)
- [Serenity](https://github.com/serenity-rs/serenity)
- [Axum](https://github.com/tokio-rs/axum)
- [MongoDB](https://www.mongodb.com/)
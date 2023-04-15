use aws_sdk_comprehend::types::{LanguageCode, SentimentType};
use aws_sdk_comprehend::Client;

// The analyze_sentiment function takes a text input and returns the detected sentiment as a String.
pub async fn analyze_sentiment(text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Initialize the AWS shared config, loading the AWS credentials and region from the environment variables
    let shared_config = aws_config::load_from_env().await;

    // Initialize the AWS Comprehend client using the shared config
    let client = Client::new(&shared_config);

    // Create a DetectSentimentRequest with the input text and language code, then send the request
    let response = client
        .detect_sentiment()
        .text(text)
        .language_code(LanguageCode::En)
        .send()
        .await?;

    // Extract the sentiment from the response and map it to a String
    let sentiment = match response.sentiment {
        Some(sentiment_type) => match sentiment_type {
            SentimentType::Mixed => "mixed",
            SentimentType::Negative => "negative",
            SentimentType::Neutral => "neutral",
            SentimentType::Positive => "positive",
            _ => "unknown",
        },
        None => "unknown",
    };

    // Return the detected sentiment as a String
    Ok(sentiment.to_string())
}

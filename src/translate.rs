use aws_sdk_translate as translate;

// This function takes a reference to a text string and returns the translated text if it
// contains more than 20 words, otherwise it returns None. If an error occurs during the
// translation process, the function returns a translate::Error.
pub async fn translate_to_ko(text: &str) -> Result<Option<String>, translate::Error> {
    // Split the input text by whitespace and count the resulting parts.
    let word_count = text.split_whitespace().count();

    // If the input text has 20 words or less, return None.
    if word_count <= 20 {
        return Ok(None);
    }

    // Load AWS shared config from environment variables.
    let shared_config = aws_config::load_from_env().await;

    // Create an Amazon Translate client.
    let client = translate::Client::new(&shared_config);

    // Send a translation request to Amazon Translate, specifying the source language code
    // as English (en) and the target language code as Korean (ko).
    let response = client
        .translate_text()
        .source_language_code("en")
        .target_language_code("ko")
        .text(text)
        .send()
        .await?;

    // Extract the translated text from the response.
    let translated_text = response.translated_text().unwrap_or_default();

    // Return the translated text as a String wrapped in Some.
    Ok(Some(translated_text.to_string()))
}

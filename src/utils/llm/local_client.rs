use ollama_rs::generation::embeddings::request::GenerateEmbeddingsRequest;

pub async fn local_embedding_client(text: String) -> Vec<f32> {
    let embedding_model = "nomic-embed-text:latest".to_string();

    let ollama = OllamaEmbeddings::default();

    let request = GenerateEmbeddingsRequest::new(embedding_model, text.into());

    let res = ollama.generate_embeddings(request).await.unwrap();

    res
}

pub fn local_client() {
    let llm_model = "gemma3:27b".to_str();
}

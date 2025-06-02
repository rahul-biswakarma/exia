use std::collections::HashMap;

use crate::utils::llm::local_client::local_embedding_client;
use futures::future::try_join_all;
use qdrant_client::{
    prelude::*,
    qdrant::{CreateCollection, Distance, PointStruct, UpsertPoints},
};
use std::error::Error;

pub async fn list_to_embedding(list: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    let collection_name = "morph_collection";

    if !client.has_collection(collection_name).await? {
        client
            .create_collection(&CreateCollection {
                collection_name: collection_name.to_string(),
                vectors_config: Some(VectorsConfig::Params(VectorParams {
                    size: 256,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
                ..Default::default()
            })
            .await?;
    }

    let tasks = list.into_iter().map(|(key, text)| {
        let client = client.clone();
        let collection_name = collection_name.to_string();

        async move {
            let embedding = local_embedding_client(&text).await;

            let point = PointStruct::new_from_id(key, embedding.clone());

            client
                .upsert_points_blocking(&UpsertPoints {
                    collection_name: collection_name.clone(),
                    wait: Some(true),
                    points: vec![point],
                    ..Default::default()
                })
                .await?;

            Ok::<(), Box<dyn Error>>(())
        }
    });

    try_join_all(tasks).await?;

    Ok(())
}

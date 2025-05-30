use super::{SupabaseClient, UISchema};
use serde_json::Value;

pub struct UISchemaService;

impl UISchemaService {
    // Save a UI schema to the database
    pub async fn save_schema(
        client: &SupabaseClient,
        schema: &UISchema,
    ) -> Result<UISchema, String> {
        let response = client.from("ui_schemas").insert(schema).await?;

        // Parse the response and return the created schema
        let schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        schemas
            .into_iter()
            .next()
            .ok_or("No schema returned from insert".to_string())
    }

    // Get all schemas for a user
    pub async fn get_user_schemas(
        client: &SupabaseClient,
        user_id: &str,
    ) -> Result<Vec<UISchema>, String> {
        let response = client
            .from("ui_schemas")
            .select("*")
            .eq("user_id", user_id)
            .order("created_at", false) // Most recent first
            .execute()
            .await?;

        let schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(schemas)
    }

    // Get public schemas (for discovery)
    pub async fn get_public_schemas(
        client: &SupabaseClient,
        limit: Option<usize>,
    ) -> Result<Vec<UISchema>, String> {
        let mut query = client
            .from("ui_schemas")
            .select("*")
            .eq("is_public", "true")
            .order("created_at", false);

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        let response = query.execute().await?;

        let schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(schemas)
    }

    // Get a specific schema by ID
    pub async fn get_schema_by_id(
        client: &SupabaseClient,
        schema_id: &str,
    ) -> Result<Option<UISchema>, String> {
        let response = client
            .from("ui_schemas")
            .select("*")
            .eq("id", schema_id)
            .execute()
            .await?;

        let schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(schemas.into_iter().next())
    }

    // Update an existing schema
    pub async fn update_schema(
        client: &SupabaseClient,
        schema_id: &str,
        updates: &UISchema,
    ) -> Result<UISchema, String> {
        let response = client
            .from("ui_schemas")
            .eq("id", schema_id)
            .update(updates)
            .await?;

        let schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        schemas
            .into_iter()
            .next()
            .ok_or("No schema returned from update".to_string())
    }

    // Delete a schema
    pub async fn delete_schema(
        client: &SupabaseClient,
        schema_id: &str,
        user_id: &str,
    ) -> Result<(), String> {
        client
            .from("ui_schemas")
            .eq("id", schema_id)
            .eq("user_id", user_id) // Ensure user can only delete their own schemas
            .delete()
            .await?;

        Ok(())
    }

    // Search schemas by title or description
    pub async fn search_schemas(
        client: &SupabaseClient,
        query: &str,
        user_id: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<UISchema>, String> {
        // For basic search, we'll use PostgREST's text search
        // In a real implementation, you might want to use full-text search
        let mut db_query = client.from("ui_schemas").select("*");

        // Add text search conditions (this is a simplified version)
        // Supabase supports more advanced text search capabilities
        if let Some(user_id) = user_id {
            db_query = db_query.eq("user_id", user_id);
        } else {
            db_query = db_query.eq("is_public", "true");
        }

        db_query = db_query.order("created_at", false);

        if let Some(limit) = limit {
            db_query = db_query.limit(limit);
        }

        let response = db_query.execute().await?;

        let mut schemas: Vec<UISchema> = serde_json::from_value(response)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Client-side filtering for simplicity
        // In production, you'd want to do this server-side
        schemas.retain(|schema| {
            schema.title.to_lowercase().contains(&query.to_lowercase())
                || schema
                    .description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query.to_lowercase()))
                    .unwrap_or(false)
                || schema.prompt.to_lowercase().contains(&query.to_lowercase())
        });

        Ok(schemas)
    }

    // Get schemas by tags
    pub async fn get_schemas_by_tags(
        client: &SupabaseClient,
        tags: &[String],
        limit: Option<usize>,
    ) -> Result<Vec<UISchema>, String> {
        // This would require PostgreSQL array operations in a real implementation
        // For now, we'll get all public schemas and filter client-side
        let schemas = Self::get_public_schemas(client, limit).await?;

        let filtered_schemas: Vec<UISchema> = schemas
            .into_iter()
            .filter(|schema| {
                tags.iter().any(|tag| {
                    schema
                        .tags
                        .iter()
                        .any(|schema_tag| schema_tag.to_lowercase() == tag.to_lowercase())
                })
            })
            .collect();

        Ok(filtered_schemas)
    }

    // Get trending/popular schemas (based on some metric)
    pub async fn get_trending_schemas(
        client: &SupabaseClient,
        limit: Option<usize>,
    ) -> Result<Vec<UISchema>, String> {
        // For now, just return recent public schemas
        // In a real implementation, you might track views, likes, etc.
        Self::get_public_schemas(client, limit).await
    }
}

// Analytics and usage tracking
pub struct AnalyticsService;

impl AnalyticsService {
    // Track schema generation
    pub async fn track_schema_generation(
        client: &SupabaseClient,
        user_id: Option<&str>,
        prompt: &str,
        schema_id: Option<&str>,
    ) -> Result<(), String> {
        let event_data = serde_json::json!({
            "event_type": "schema_generated",
            "user_id": user_id,
            "prompt": prompt,
            "schema_id": schema_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        client.from("analytics_events").insert(&event_data).await?;

        Ok(())
    }

    // Track schema usage/view
    pub async fn track_schema_view(
        client: &SupabaseClient,
        schema_id: &str,
        user_id: Option<&str>,
    ) -> Result<(), String> {
        let event_data = serde_json::json!({
            "event_type": "schema_viewed",
            "user_id": user_id,
            "schema_id": schema_id,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        client.from("analytics_events").insert(&event_data).await?;

        Ok(())
    }
}

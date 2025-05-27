mod compiler;
mod llm;
mod models;
mod storage;
mod ui;

use anyhow::Result;
use std::env;
use tracing::{error, info};
use tracing_subscriber;

use ui::{App, UI};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Check for API key and provide helpful message
    if env::var("GEMINI_API_KEY").is_err() {
        eprintln!("⚠️  Warning: GEMINI_API_KEY environment variable not set.");
        eprintln!("   The application will work in offline mode with limited functionality.");
        eprintln!("   To enable AI features, set your Gemini API key:");
        eprintln!("   export GEMINI_API_KEY=your_api_key_here");
        eprintln!();
    }

    info!("Starting DSA Learning Assistant");

    // Initialize the application
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to initialize application: {}", e);
            eprintln!("❌ Error: Failed to initialize application: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize the UI
    let mut ui = match UI::new() {
        Ok(ui) => ui,
        Err(e) => {
            error!("Failed to initialize UI: {}", e);
            eprintln!("❌ Error: Failed to initialize terminal UI: {}", e);
            std::process::exit(1);
        }
    };

    // Show welcome message
    println!("🎯 Welcome to DSA Learning Assistant!");
    println!("   Your personalized Rust DSA practice companion");
    println!();

    // Main application loop
    loop {
        // Draw the UI
        if let Err(e) = ui.draw(&mut app) {
            error!("Failed to draw UI: {}", e);
            break;
        }

        // Handle events
        if let Ok(Some(event)) = ui.handle_events() {
            if let Err(e) = app.handle_event(event).await {
                error!("Error handling event: {}", e);
                // Don't break on event handling errors, just log them
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }

        // Small delay to prevent excessive CPU usage
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    info!("DSA Learning Assistant shutting down");
    println!("👋 Thanks for using DSA Learning Assistant!");
    println!("   Keep practicing and happy coding! 🦀");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_initialization() {
        // Test that the app can be initialized without panicking
        let result = App::new();
        assert!(result.is_ok(), "App should initialize successfully");
    }

    #[test]
    fn test_environment_setup() {
        // Test that the application handles missing environment variables gracefully
        std::env::remove_var("GEMINI_API_KEY");
        let result = App::new();
        assert!(result.is_ok(), "App should work without API key");
    }
}

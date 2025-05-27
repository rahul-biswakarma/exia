use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

// Import the widgets - note: you'll need to adjust these imports based on your actual crate structure
use crate::models::LLMUsage;
use crate::ui::widgets::{LLMInfoWidget, LLMStreamInfo, LLMStreamStatus, Widget};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create different LLM stream states for demonstration
    let mut demo_states = create_demo_states();
    let mut current_state = 0;
    let mut animation_frame = 0;

    loop {
        // Update animation frame
        animation_frame = (animation_frame + 1) % 100;

        // Render the current state
        terminal.draw(|f| {
            let area = f.size();
            render_demo(f, area, &demo_states[current_state], animation_frame);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right | KeyCode::Char('n') => {
                        current_state = (current_state + 1) % demo_states.len();
                    }
                    KeyCode::Left | KeyCode::Char('p') => {
                        current_state = if current_state == 0 {
                            demo_states.len() - 1
                        } else {
                            current_state - 1
                        };
                    }
                    _ => {}
                }
            }
        }

        // Update streaming content for streaming state
        if current_state == 2 {
            update_streaming_content(&mut demo_states[current_state]);
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn create_demo_states() -> Vec<Option<LLMStreamInfo>> {
    vec![
        // State 1: No LLM stream (empty state)
        None,
        // State 2: Initializing
        Some({
            let mut info = LLMStreamInfo::new("Code Analysis".to_string());
            info.status = LLMStreamStatus::Initializing;
            info.progress = 0.0;
            info.model_name = Some("gemini-1.5-flash".to_string());
            info
        }),
        // State 3: Streaming
        Some({
            let mut info = LLMStreamInfo::new("Algorithm Optimization".to_string());
            info.status = LLMStreamStatus::Streaming;
            info.progress = 0.65;
            info.input_tokens = Some(1250);
            info.output_tokens = Some(890);
            info.estimated_cost_usd = Some(0.0003);
            info.model_name = Some("gemini-1.5-flash".to_string());
            info.streamed_content = "Analyzing your algorithm implementation...\n\nI can see you're using a dynamic programming approach, which is excellent for this type of problem. Here are my observations:\n\n1. Time Complexity: O(n²) - This is optimal for the given constraints\n2. Space Complexity: O(n²) - Could be optimized to O(n)\n3. Edge Cases: Consider handling empty arrays\n\nLet me suggest some optimizations...".to_string();
            info
        }),
        // State 4: Complete
        Some({
            let mut info = LLMStreamInfo::new("Code Review".to_string());
            info.status = LLMStreamStatus::Complete;
            info.progress = 1.0;
            info.input_tokens = Some(2100);
            info.output_tokens = Some(1450);
            info.estimated_cost_usd = Some(0.0008);
            info.model_name = Some("gemini-1.5-pro".to_string());
            info.streamed_content = "Code review completed successfully!\n\nYour implementation demonstrates solid understanding of algorithmic principles. The solution is correct and efficient.\n\nKey strengths:\n✓ Proper time complexity\n✓ Clean, readable code\n✓ Good variable naming\n✓ Handles edge cases\n\nSuggestions for improvement:\n• Add inline comments for complex logic\n• Consider using more descriptive function names\n• Add input validation\n\nOverall score: 8.5/10\nExcellent work!".to_string();
            info
        }),
        // State 5: Error
        Some({
            let mut info = LLMStreamInfo::new("Syntax Check".to_string());
            info.status = LLMStreamStatus::Error;
            info.progress = 0.3;
            info.model_name = Some("gemini-1.5-flash".to_string());
            info.error_message =
                Some("API rate limit exceeded. Please try again in a few moments.".to_string());
            info
        }),
    ]
}

fn update_streaming_content(stream_info: &mut Option<LLMStreamInfo>) {
    if let Some(ref mut info) = stream_info {
        if info.status == LLMStreamStatus::Streaming {
            // Simulate streaming by gradually increasing progress and content
            info.progress = (info.progress + 0.01).min(0.95);

            // Add more content occasionally
            if rand::random::<f64>() < 0.1 {
                let additional_content = [
                    "\n\nAnalyzing memory usage patterns...",
                    "\n\nChecking for potential optimizations...",
                    "\n\nEvaluating algorithmic complexity...",
                    "\n\nGenerating performance recommendations...",
                ];
                let content =
                    additional_content[rand::random::<usize>() % additional_content.len()];
                info.append_content(content);
            }
        }
    }
}

fn render_demo(
    f: &mut Frame,
    area: Rect,
    stream_info: &Option<LLMStreamInfo>,
    animation_frame: usize,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Instructions
            Constraint::Min(0),    // LLM widget
        ])
        .split(area);

    // Instructions
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Paragraph;
    let instructions =
        Paragraph::new("LLM Widget Demo - Press 'n'/'p' for next/previous state, 'q' to quit")
            .style(Style::default().fg(Color::Yellow));
    f.render_widget(instructions, chunks[0]);

    // LLM widget
    let llm_widget = LLMInfoWidget::new(stream_info.as_ref()).with_animation_frame(animation_frame);
    llm_widget.render(f, chunks[1]);
}

// Mock rand function for demo
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random<T>() -> T
    where
        T: From<u64>,
    {
        let mut hasher = DefaultHasher::new();
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);
        T::from(hasher.finish())
    }
}

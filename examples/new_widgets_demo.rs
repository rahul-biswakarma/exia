// Note: For examples in a binary crate, we need to reference the modules directly
// In a real application, these would be available through the crate structure
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::collections::{HashMap, VecDeque};
use std::io;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create sample data
    let stats = create_sample_statistics();
    let cost_analytics = create_sample_cost_analytics();
    let llm_stream_info = create_sample_llm_stream_info();
    let system_metrics = create_sample_system_metrics();
    let (cpu_history, ram_history) = create_sample_system_history();

    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create the home layout widget with LLM stream info
    let home_layout = HomeLayoutWidget::new(&stats, &cpu_history, &ram_history)
        .with_cost_analytics(Some(&cost_analytics))
        .with_llm_stream_info(Some(&llm_stream_info))
        .with_system_metrics(Some(&system_metrics))
        .with_animation_frame(0);

    // Render the layout
    terminal.draw(|f| {
        let area = f.size();
        home_layout.render(f, area);
    })?;

    // Wait for user input
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn create_sample_statistics() -> Statistics {
    let mut topic_distribution = HashMap::new();
    topic_distribution.insert(Topic::Arrays, 15);
    topic_distribution.insert(Topic::LinkedLists, 8);
    topic_distribution.insert(Topic::Trees, 12);
    topic_distribution.insert(Topic::DynamicProgramming, 5);
    topic_distribution.insert(Topic::Graphs, 7);

    let mut difficulty_distribution = HashMap::new();
    difficulty_distribution.insert(Difficulty::Easy, 20);
    difficulty_distribution.insert(Difficulty::Medium, 15);
    difficulty_distribution.insert(Difficulty::Hard, 8);

    Statistics {
        total_questions: 43,
        total_solutions: 47,
        accepted_solutions: 35,
        success_rate: 74.5,
        avg_execution_time: 125.7,
        topic_distribution,
        difficulty_distribution,
        current_streak: 5,
    }
}

fn create_sample_cost_analytics() -> CostAnalytics {
    use std::collections::HashMap;

    let mut cost_by_model = HashMap::new();
    cost_by_model.insert("gemini-1.5-flash".to_string(), 0.0012);
    cost_by_model.insert("gemini-1.5-pro".to_string(), 0.0006);

    let mut cost_by_request_type = HashMap::new();
    cost_by_request_type.insert("question_generation".to_string(), 0.0008);
    cost_by_request_type.insert("hint_generation".to_string(), 0.0005);
    cost_by_request_type.insert("feedback_generation".to_string(), 0.0005);

    CostAnalytics {
        total_cost_usd: 0.0018,
        cost_by_model,
        cost_by_request_type,
        tokens_used: 7963,
        requests_count: 9,
        average_cost_per_request: 0.0002,
        cost_trend: vec![], // Empty for demo
    }
}

fn create_sample_llm_stream_info() -> LLMStreamInfo {
    let mut stream_info = LLMStreamInfo::new("Code Analysis".to_string());
    stream_info.status = LLMStreamStatus::Streaming;
    stream_info.progress = 0.65;
    stream_info.input_tokens = Some(1250);
    stream_info.output_tokens = Some(890);
    stream_info.estimated_cost_usd = Some(0.0003);
    stream_info.model_name = Some("gemini-1.5-flash".to_string());
    stream_info.streamed_content = "Analyzing your code structure...\n\nThe algorithm you've implemented shows good understanding of dynamic programming principles. Here are some observations:\n\n1. Time complexity is O(n²) which is optimal for this problem\n2. Space complexity could be optimized from O(n²) to O(n)\n3. Consider edge cases for empty input".to_string();
    stream_info
}

fn create_sample_system_metrics() -> SystemMetrics {
    SystemMetrics {
        cpu_usage: 45.2,
        ram_usage: 2.8, // GB
        ram_total: 8.0, // GB
        timestamp: Instant::now(),
    }
}

fn create_sample_system_history() -> (VecDeque<(f64, f64)>, VecDeque<(f64, f64)>) {
    let mut cpu_history = VecDeque::new();
    let mut ram_history = VecDeque::new();

    // Generate sample data for the last 60 seconds
    for i in 0..60 {
        let time = i as f64;

        // CPU usage with some variation
        let cpu_base = 45.0;
        let cpu_variation = 10.0 * (time * 0.1).sin();
        let cpu_usage = (cpu_base + cpu_variation).max(0.0).min(100.0);
        cpu_history.push_back((time, cpu_usage));

        // RAM usage (percentage)
        let ram_base = 35.0; // 35% of 8GB
        let ram_variation = 5.0 * (time * 0.05).cos();
        let ram_usage = (ram_base + ram_variation).max(0.0).min(100.0);
        ram_history.push_back((time, ram_usage));
    }

    (cpu_history, ram_history)
}

// Example of how to use individual widgets
#[allow(dead_code)]
fn demo_individual_widgets() {
    let stats = create_sample_statistics();
    let cost_analytics = create_sample_cost_analytics();
    let llm_stream_info = create_sample_llm_stream_info();
    let system_metrics = create_sample_system_metrics();
    let (cpu_history, ram_history) = create_sample_system_history();

    // Individual widget usage examples:

    // 1. Learning Efficiency Widget
    let _learning_efficiency = LearningEfficiencyWidget::new(&stats);

    // 2. Learning Unit Status Widget
    let _learning_unit_status = LearningUnitStatusWidget::new(&stats);

    // 3. System Monitor Widget
    let _system_monitor = SystemMonitorWidget::new(
        Some(&cost_analytics),
        &cpu_history,
        &ram_history,
        Some(&system_metrics),
    );

    // 4. LLM Info Widget
    let _llm_info = LLMInfoWidget::new(Some(&llm_stream_info)).with_animation_frame(0);

    // These widgets can be rendered individually in any layout you prefer
}

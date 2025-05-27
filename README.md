# 🎯 DSA Learning Assistant

A comprehensive **Data Structures and Algorithms** learning companion built in **Rust** with **AI-powered question generation**, **real-time code compilation**, and **personalized progress tracking**.

## ✨ Features

### 🤖 AI-Powered Learning
- **Adaptive Question Generation**: Uses Google's Gemini LLM to generate personalized DSA questions based on your learning progress
- **Smart Difficulty Scaling**: Questions adapt to your skill level and focus on weak areas
- **Intelligent Hints**: Get contextual hints for your current code attempts
- **Detailed Feedback**: Receive comprehensive analysis of your solutions with improvement suggestions

### 💻 Code Execution & Testing
- **Real-time Compilation**: Compile and test your Rust solutions instantly
- **Comprehensive Testing**: Automatic test case execution with detailed results
- **Error Analysis**: Clear compilation and runtime error reporting
- **Performance Metrics**: Track execution time and memory usage

### 📊 Progress Tracking
- **Learning Analytics**: Detailed statistics on your problem-solving journey
- **Topic Mastery**: Track progress across different DSA topics (Arrays, Trees, Graphs, etc.)
- **Difficulty Progression**: Monitor improvement across Easy, Medium, and Hard problems
- **Streak Tracking**: Maintain your solving streak for motivation
- **Session History**: Review past learning sessions and solutions

### 🎨 Beautiful Terminal UI
- **Modern Interface**: Clean, colorful terminal UI built with `ratatui`
- **Intuitive Navigation**: Easy keyboard shortcuts and navigation
- **Responsive Design**: Adapts to different terminal sizes
- **Real-time Updates**: Live feedback and status updates
- **Multiple Views**: Home, Question, Code Editor, Results, Statistics, and Help screens

### 💾 Data Persistence
- **Local Storage**: All your progress is saved locally in JSON format
- **Session Management**: Track individual learning sessions
- **Export/Import**: Backup and restore your learning data
- **Cross-platform**: Works on macOS, Linux, and Windows

## 🚀 Quick Start

### Prerequisites
- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **Google Gemini API Key** (for AI features)

### Installation

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd dsa_learning_assistant
   ```

2. **Set up your Gemini API Key**:
   ```bash
   export GEMINI_API_KEY="your_gemini_api_key_here"
   ```

   To get a Gemini API key:
   - Visit [Google AI Studio](https://makersuite.google.com/app/apikey)
   - Create a new API key
   - Copy and set it as an environment variable

3. **Build and run**:
   ```bash
   cargo build --release
   cargo run
   ```

### First Run

1. The application will create a data directory at `~/.local/share/dsa_learning_assistant/`
2. Start with generating your first question by pressing `g` or selecting "Generate New Question"
3. Follow the on-screen instructions to navigate through the interface

## 🎮 Usage Guide

### Navigation
- **Tab/Shift+Tab**: Navigate between sections
- **↑/↓**: Scroll up/down
- **Enter**: Select/Confirm
- **Esc**: Go back/Cancel
- **q**: Quit application

### Home Screen
- **g**: Generate new question
- **r**: View recent questions
- **s**: View statistics
- **h**: Show help

### Question View
- **c**: Start coding
- **h**: Show/hide hints
- **n/p**: Next/previous hint

### Code Editor
- **Ctrl+S**: Submit solution
- **Ctrl+H**: Get hint for current code
- **Ctrl+C**: Clear editor
- **Esc**: Go back to question

### Results Screen
- **f**: Get detailed feedback
- **r**: Retry question
- **n**: Next question

## 🏗️ Architecture

The application is built with a modular architecture:

```
src/
├── main.rs              # Application entry point
├── models/              # Data structures and types
├── llm/                 # Gemini LLM integration
├── compiler/            # Rust code compilation and execution
├── storage/             # Data persistence layer
└── ui/                  # Terminal user interface
    ├── app.rs          # Application state management
    ├── mod.rs          # UI rendering logic
    └── components/     # Reusable UI components
```

### Key Components

- **Models**: Define data structures for questions, solutions, progress tracking
- **LLM Module**: Handles communication with Google's Gemini API
- **Compiler**: Manages Rust code compilation and test execution
- **Storage**: Provides persistent data storage using JSON files
- **UI**: Beautiful terminal interface using `ratatui` and `crossterm`

## 🔧 Configuration

### Environment Variables
- `GEMINI_API_KEY`: Your Google Gemini API key (required for AI features)

### Data Directory
- **macOS/Linux**: `~/.local/share/dsa_learning_assistant/`
- **Windows**: `%APPDATA%\dsa_learning_assistant\`

### Files
- `database.json`: Main data file containing questions, solutions, and progress
- Temporary compilation files are created and cleaned up automatically

## 🧪 Testing

Run the test suite:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## 📈 Learning Topics Covered

The AI generates questions across various DSA topics:

- **Arrays & Strings**: Manipulation, searching, sorting
- **Linked Lists**: Singly, doubly, circular lists
- **Stacks & Queues**: Implementation and applications
- **Trees**: Binary trees, BST, AVL, traversals
- **Graphs**: DFS, BFS, shortest paths, MST
- **Dynamic Programming**: Memoization, tabulation
- **Sorting & Searching**: Various algorithms and optimizations
- **Hashing**: Hash tables, collision resolution
- **Recursion & Backtracking**: Problem-solving techniques
- **Greedy Algorithms**: Optimization problems
- **Mathematical Problems**: Number theory, combinatorics

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Commit your changes**: `git commit -m 'Add amazing feature'`
4. **Push to the branch**: `git push origin feature/amazing-feature`
5. **Open a Pull Request**

### Development Setup
```bash
# Install development dependencies
cargo install cargo-watch

# Run with auto-reload during development
cargo watch -x run

# Run tests continuously
cargo watch -x test
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Google Gemini**: For providing the AI capabilities
- **Ratatui**: For the excellent terminal UI framework
- **Rust Community**: For the amazing ecosystem and tools

## 🐛 Troubleshooting

### Common Issues

1. **"GEMINI_API_KEY not set"**
   - Set your API key: `export GEMINI_API_KEY="your_key"`
   - The app works in offline mode without the API key

2. **Compilation errors**
   - Ensure you have the latest Rust version: `rustup update`
   - Check that Cargo is in your PATH

3. **Terminal display issues**
   - Ensure your terminal supports colors and Unicode
   - Try resizing your terminal window

4. **Permission errors**
   - Check write permissions for the data directory
   - On Unix systems: `chmod 755 ~/.local/share/dsa_learning_assistant/`

### Getting Help

- **GitHub Issues**: Report bugs and request features
- **Discussions**: Ask questions and share ideas
- **Documentation**: Check the inline help (`h` key in the app)

---

**Happy Coding! 🦀✨**

*Built with ❤️ in Rust*

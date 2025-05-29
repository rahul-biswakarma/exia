# 🧠 Synapse - AI-Powered Dynamic UI Generation

Synapse is a revolutionary feature within Exia that allows you to generate custom user interfaces on-the-fly using natural language requests. Simply describe what you want, and Synapse will create the appropriate UI components for you.

## ✨ Features

### Supported UI Components

- **Labels** - Display text content with customizable styling
- **Checkboxes** - Interactive checkboxes with state management
- **Switches** - Toggle switches for binary options
- **Avatars** - User profile images with size customization
- **Progress Bars** - Visual progress indicators with value/max properties
- **Sliders** - Range input controls with min/max/step configuration
- **Separators** - Visual dividers for content organization
- **Portals** - Container elements for grouping content
- **Grid Layouts** - Responsive grid systems for complex arrangements

### Smart Layout System

Synapse uses an intelligent grid-based layout system that automatically organizes components based on your request. The system supports:

- **Responsive Grids** - Automatic row/column calculation
- **Flexible Spacing** - Customizable gaps and padding
- **Span Control** - Components can span multiple rows/columns
- **Nested Layouts** - Complex hierarchical arrangements

## 🚀 Getting Started

### Accessing Synapse

1. **From Settings**: Click the "🧠 Go to Synapse" button
2. **Direct URL**: Navigate to `/synapse` in your browser
3. **Navigation**: Use the navigation links between Settings and Synapse

### Using Synapse

1. **Enter Your Request**: Type what you want to create in the text area
2. **Generate**: Click "✨ Generate UI" to create your interface
3. **View Results**: See your custom UI rendered in real-time
4. **Inspect Schema**: Use "📋 View Schema" to see the underlying JSON
5. **Clear & Restart**: Use "🗑️ Clear" to start over

## 💡 Example Requests

### Educational Content
- "Create a quiz app to test my knowledge"
- "Build a flashcard system for learning"
- "Make a progress tracker for my studies"

### Productivity Tools
- "Build a simple calculator"
- "Create a todo list with priorities"
- "Design a time tracker interface"

### Interactive Applications
- "Create a snake game"
- "Build a color picker tool"
- "Make a survey form"

### Data Visualization
- "Design a weather widget"
- "Create a dashboard with metrics"
- "Build a progress monitoring system"

## 🔧 Technical Architecture

### Core Components

#### Schema Parser (`schema_parser.rs`)
- Handles JSON schema parsing and validation
- Supports untagged enum deserialization for flexible UI definitions
- Provides type-safe parsing of UI elements and layouts

#### Renderer (`renderer.rs`)
- Converts JSON schemas into Dioxus UI components
- Handles both atomic components and complex layouts
- Provides error handling and fallback rendering

#### LLM Client (`client.rs`)
- Manages AI model interactions for UI generation
- Includes mock responses for development
- Extensible for real LLM integration

### Schema Format

Synapse uses a JSON schema format to define UI components:

```json
{
  "uiElements": [
    {
      "type": "label",
      "properties": {
        "text": "Hello World"
      }
    },
    {
      "layoutType": "grid",
      "rows": 2,
      "cols": 2,
      "gap": 10,
      "padding": 20,
      "elements": [
        {
          "atom": {
            "type": "checkbox",
            "properties": {
              "checked": false,
              "value": "option1"
            }
          },
          "row": 1,
          "col": 1
        }
      ]
    }
  ]
}
```

### Component Properties

#### Label
- `text`: String content to display

#### Checkbox/Switch
- `checked`: Boolean state (default: false)
- `disabled`: Boolean disabled state (default: false)
- `value`: String value identifier
- `id`: String element ID

#### Avatar
- `src`: Image source URL
- `alt`: Alternative text
- `size`: Number pixel size (default: 32)

#### Progress
- `value`: Number current value
- `max`: Number maximum value

#### Slider
- `value`: Number current value
- `min`: Number minimum value
- `max`: Number maximum value
- `step`: Number step increment
- `disabled`: Boolean disabled state
- `id`: String element ID

#### Grid Layout
- `layoutType`: "grid"
- `rows`: Number of rows
- `cols`: Number of columns
- `gap`: Optional spacing between elements
- `padding`: Optional internal padding
- `elements`: Array of positioned elements

#### Grid Element
- `atom`: The UI component to render
- `row`: Grid row position (1-indexed)
- `col`: Grid column position (1-indexed)
- `rowSpan`: Optional row span (default: 1)
- `colSpan`: Optional column span (default: 1)

## 🧪 Testing

Synapse includes comprehensive test coverage:

- **Schema Parser Tests**: Validate JSON parsing and error handling
- **Renderer Tests**: Ensure proper UI component generation
- **Integration Tests**: Test end-to-end functionality

Run tests with:
```bash
cargo test synapse
```

## 🔮 Future Enhancements

### Planned Features
- **Real LLM Integration** - Connect to OpenAI, Anthropic, or Google Gemini
- **Interactive State Management** - Stateful components with event handling
- **Component Library** - Pre-built component templates
- **Export Functionality** - Save generated UIs as standalone components
- **Theme System** - Customizable styling and themes
- **Animation Support** - Animated transitions and effects

### Advanced Capabilities
- **Multi-page Applications** - Generate complete app flows
- **Data Binding** - Connect to external data sources
- **Real-time Collaboration** - Share and edit UIs with others
- **Version Control** - Track UI generation history
- **A/B Testing** - Generate multiple UI variants

## 🤝 Contributing

To contribute to Synapse development:

1. **Add New Components**: Update schema parser and renderer
2. **Enhance LLM Integration**: Improve prompt engineering
3. **Expand Test Coverage**: Add comprehensive test cases
4. **Improve Documentation**: Update examples and guides

### Development Workflow

1. Update `schema_parser.rs` for new component types
2. Add rendering logic in `renderer.rs`
3. Update LLM prompts to include new components
4. Add tests in `src/components/synapse/tests/`
5. Update documentation and examples

## 📝 License

Synapse is part of the Exia project and follows the same licensing terms.

---

**Ready to create something amazing? Head to `/synapse` and start building!** 🚀

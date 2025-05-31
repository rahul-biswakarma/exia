# 🧠 Hybrid LLM System for Scalable Personal Assistant Architecture

This system is designed to power a context-aware AI assistant that uses a combination of **cloud LLMs**, **local LLMs**, and a **dual-database strategy** (conventional and structured) to efficiently process, store, and act on user interactions and insights.

> It’s a hybrid design that prioritizes cost-efficiency, user control, and long-term scalability.

---

## 📚 System Overview

The assistant logs and analyzes **sessions** from various subsystems like Synapse, Planner, Nudge, etc.

### ✨ Key Concepts

| Component              | Purpose                                                                 |
|------------------------|-------------------------------------------------------------------------|
| `Summarizer`           | Generates rich, natural language summary using cloud LLM                |
| `Summary DB`           | Stores raw summaries + metadata for future re-processing                |
| `Translator`           | Uses local LLM to convert summary into structured JSONs for each system |
| `System DBs`           | Stores each system’s pre-formatted JSON (Synapse, Planner, etc.)        |
| `Refresher UI`         | UI control to re-trigger local LLM parsing when needed                  |
| `Model Registry`       | Tracks model and prompt version used for traceability                   |

---

## 🧩 Architecture Diagram (Mermaid)

```mermaid
graph TD
  subgraph 🧠 Cloud
    A[User Session] --> B[Summarizer (Cloud LLM)]
  end

  subgraph 📦 Summary Store
    B --> C[Summary DB]
  end

  subgraph 🧰 Local Engine
    C --> D[Translator (Local LLM)]
    D --> E1[Planner JSON]
    D --> E2[Synapse JSON]
    D --> E3[Nudge JSON]
  end

  subgraph 🗃️ System Databases
    E1 --> F1[Planner DB]
    E2 --> F2[Synapse DB]
    E3 --> F3[Nudge DB]
  end

  subgraph 🧑‍💻 UI Layer
    G[User Dashboard]
    G -->|Refresh| D
    G -->|Read| F1 & F2 & F3
  end
```

---

## 🚀 Flow Summary

1. **Session Trigger**: A user interacts with the assistant (e.g., via Synapse).
2. **Cloud Summary**: A cloud-hosted LLM generates a rich session summary.
3. **Stored Raw**: The summary is stored in a flat format in the `Summary DB` for future decoding.
4. **Translation**: On-demand, a local LLM translates the raw summary into system-specific JSONs.
5. **Storage & Access**: These JSONs are stored in structured system-specific databases.
6. **User Control**: From the dashboard, the user can refresh or re-translate summaries anytime.

---

## 🧠 Why This Architecture?

### ✅ Benefits

- **Scalable**: Doesn’t lock you into a rigid schema — summaries are always re-generatable.
- **Cost-Efficient**: Expensive cloud LLMs used only once per session; translation happens locally.
- **User-Friendly**: Summaries are readable by both machines and humans.
- **Modular**: Easy to add new systems (e.g., Fitness, Health, Learning) by defining JSON rules.

### ⚠️ Trade-offs

| Concern             | Mitigation Strategy                                           |
|---------------------|---------------------------------------------------------------|
| Cloud LLM Cost      | Limit use to first-time summarization                         |
| Local Model Quality | Fine-tune or use robust open-weight models for translation    |
| JSON Drift          | Maintain versioning and test suite per system                 |
| Re-translation Delay| Make async or use queues if scaling is needed                 |

---

## 🗂 Suggested File Location

Put this file in:

```
/source/meta/session-summary-pipeline.md
```

> Why? It’s not a component or capability, but a system-wide meta layer defining how data flows. `meta/` or `docs/` makes it clear this governs orchestration, not business logic.

---

## 🧪 Future Improvements

- [ ] Add embedding-based similarity search on `Summary DB` (optional vector support)
- [ ] Auto-versioning of models and prompts used per summary
- [ ] System JSON diffing with logs when format changes
- [ ] Real-time streaming dashboard updates using WebSockets

---

## 📌 Naming Recommendation

Save this file as:

```
session-summary-pipeline.md
```

> It best captures the essence of this system — how raw session summaries move through your assistant’s infrastructure.

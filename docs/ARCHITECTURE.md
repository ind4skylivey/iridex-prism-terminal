# Architecture Overview

## High-Level Modules
- `core`: Theme parsing, validation, prompt generation, application to shells.
- `context`: Git/project/time/system detectors + rule engine.
- `widgets`: Async widget runtime for animated prompt components.
- `tui`: Ratatui-based preview/editor.
- `sync`: Cloud sync client + storage + dotfiles.
- `daemon`: Auto-switch background watcher plus IPC stub.
- `cli`: Clap-powered command router.

## Data Flow
1. User invokes CLI → Clap routes to handler.
2. Handler loads theme (core::loader) and optionally context snapshot.
3. TUI renders via ratatui (preview) or future editor.
4. `core::apply` writes prompt script and injects into shell rc file.
5. Daemon monitors context (tokio interval) and reuses loader/apply.
6. Sync operations serialize data via serde/toml/json + reqwest.

## Context Evaluation
```
ContextDetector
 ├─ git::detect_git_context (branch/conflicts/dirtiness)
 ├─ project::detect_project_context (language heuristics)
 ├─ time::detect_time_context (chrono)
 └─ system::detect_system_context (sysinfo)
RuleEngine -> Option<String> theme_name
```

## Widgets Runtime
```
WidgetManager
 ├─ Vec<Arc<Mutex<Box<dyn Widget>>>>
 └─ render_all() sequentially renders each widget (async-ready)
```
Widgets use `colored` for ANSI styling and keep minimal state.

## AI Feature Flag
- Feature: `ai_small_model`.
- Env gate: `PRISM_AI=1` to enable Ollama-backed suggestions (future work).

## Plan TODOs
The `PLAN DE TRABAJO DETALLADO` checkpoints are referenced as TODO lists throughout source modules and documentation to keep sprints visible inside the repo.

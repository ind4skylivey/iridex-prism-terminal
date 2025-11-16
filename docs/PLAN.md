# Work Plan Checklist

## Phase 1 – Foundation (Weeks 1-2)
### Sprint 1.1 – Setup & Core Infrastructure
- [x] Initialize Cargo workspace
- [x] Configure dependencies
- [x] Directory structure ready
- [x] Logging system
- [x] Custom error handling

### Sprint 1.2 – Theme Engine
- [x] Theme TOML parser
- [x] Color palette management
- [x] Prompt generation (Zsh/Bash/Fish)
- [x] Theme validation
- [x] Built-in themes (5 presets)

### Sprint 1.3 – Shell Integration
- [x] Apply mechanism for Zsh
- [x] Apply mechanism for Bash
- [x] Apply mechanism for Fish
- [x] Backup & rollback
- [x] Multi-shell tests

## Phase 2 – Context & Intelligence (Weeks 2-3)
### Sprint 2.1 – Context Detection
- [x] Git detector
- [x] Project type detector
- [x] Time detection
- [x] System load monitoring
- [x] Docker detection

### Sprint 2.2 – Auto-Switching Logic
- [x] Rule engine
- [x] Priority system
- [x] Context-aware switching
- [x] `rules.toml`
- [x] Manual override

### Sprint 2.3 – AI (Small Model)
- [ ] Ollama integration *(planned for a future release, not part of v0.1.1)*
- [ ] Profile analyzer *(future release)*
- [ ] Non-LLM suggestion engine *(future release)*
- [ ] Heuristic scoring *(future release)*
- [ ] Smart recommendations *(future release)*

## Phase 3 – Widgets & Live Data (Weeks 3-4)
### Sprint 3.1 – Widget System
- [x] Widget trait/runtime
- [x] Git widget
- [x] System widget
- [x] Clock widget
- [x] Docker widget

### Sprint 3.2 – Advanced Widgets
- [x] Custom widget support
- [x] Widget configuration
- [x] Performance optimizations
- [x] Animation system
- [x] Plugin SDK

### Sprint 3.3 – Prompt Integration
- [x] Real-time prompt rendering
- [x] Widget streaming updates
- [x] Performance tuning
- [x] Caching
- [x] Battery optimization

## Phase 4 – TUI & Preview (Weeks 4-5)
### Sprint 4.1 – Preview Interface
- [x] Ratatui setup
- [x] Terminal frame renderer
- [x] Live preview
- [x] Color picker

### Sprint 4.2 – Interactive Editor
- [x] Theme editor TUI
- [x] Real-time color adjustments
- [x] Segment customization
- [x] Save & apply from editor

### Sprint 4.3 – Theme Gallery
- [x] Browser TUI
- [x] Search/filter
- [x] Preview from gallery
- [x] Community showcase

## Phase 5 – Cloud Sync (Weeks 5-6)
### Sprint 5.1 – Sync Infrastructure
- [x] API client
- [x] JWT auth
- [x] Local storage
- [x] Conflict resolution

### Sprint 5.2 – Sync Operations
- [x] Push
- [x] Pull
- [x] Status
- [x] History/versioning
- [x] Rollback

### Sprint 5.3 – Dotfiles Manager
- [x] Tracking
- [x] Selective sync
- [x] Restore on new machine
- [x] Backup before restore
- [x] Exclusions

## Phase 6 – CLI & Polish (Weeks 6-7)
### Sprint 6.1 – CLI Commands
- [x] apply/preview/list/edit/auto
- [x] widget commands
- [x] sync commands
- [x] daemon commands
- [x] config commands

### Sprint 6.2 – Daemon & Background
- [x] Context watcher
- [x] Auto-switch daemon
- [x] Widget updater
- [x] IPC
- [x] Systemd integration

### Sprint 6.3 – Testing & Release
- [x] Unit tests
- [x] Integration tests
- [x] Performance benchmarks
- [x] Documentation
- [x] Release v0.1.0-beta
- [x] Release v0.1.1 (ready for GitHub tag/artifacts)

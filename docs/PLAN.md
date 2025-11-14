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
- [ ] Multi-shell tests

## Phase 2 – Context & Intelligence (Weeks 2-3)
### Sprint 2.1 – Context Detection
- [x] Git detector
- [x] Project type detector
- [x] Time detection
- [x] System load monitoring
- [ ] Docker detection

### Sprint 2.2 – Auto-Switching Logic
- [x] Rule engine
- [ ] Priority system
- [x] Context-aware switching
- [ ] `rules.toml`
- [ ] Manual override

### Sprint 2.3 – AI (Small Model)
- [ ] Ollama integration
- [ ] Profile analyzer
- [ ] Non-LLM suggestion engine
- [ ] Heuristic scoring
- [ ] Smart recommendations

## Phase 3 – Widgets & Live Data (Weeks 3-4)
### Sprint 3.1 – Widget System
- [x] Widget trait/runtime
- [x] Git widget
- [x] System widget
- [x] Clock widget
- [x] Docker widget

### Sprint 3.2 – Advanced Widgets
- [x] Custom widget support
- [ ] Widget configuration
- [ ] Performance optimizations
- [ ] Animation system
- [ ] Plugin SDK

### Sprint 3.3 – Prompt Integration
- [ ] Real-time prompt rendering
- [ ] Widget streaming updates
- [ ] Performance tuning
- [ ] Caching
- [ ] Battery optimization

## Phase 4 – TUI & Preview (Weeks 4-5)
### Sprint 4.1 – Preview Interface
- [x] Ratatui setup
- [x] Terminal frame renderer
- [x] Live preview
- [x] Color picker

### Sprint 4.2 – Interactive Editor
- [ ] Theme editor TUI
- [ ] Real-time color adjustments
- [ ] Segment customization
- [ ] Save & apply from editor

### Sprint 4.3 – Theme Gallery
- [ ] Browser TUI
- [ ] Search/filter
- [ ] Preview from gallery
- [ ] Community showcase

## Phase 5 – Cloud Sync (Weeks 5-6)
### Sprint 5.1 – Sync Infrastructure
- [x] API client
- [ ] JWT auth
- [x] Local storage
- [ ] Conflict resolution

### Sprint 5.2 – Sync Operations
- [x] Push
- [x] Pull
- [x] Status
- [ ] History/versioning
- [ ] Rollback

### Sprint 5.3 – Dotfiles Manager
- [x] Tracking
- [ ] Selective sync
- [ ] Restore on new machine
- [ ] Backup before restore
- [ ] Exclusions

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
- [ ] Systemd integration

### Sprint 6.3 – Testing & Release
- [ ] Unit tests
- [x] Integration tests
- [ ] Performance benchmarks
- [x] Documentation
- [ ] Release v0.1.0-beta

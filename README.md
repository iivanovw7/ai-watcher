# AI Watcher

Tui application written in Rust that monitors the focused Hyprland monitor for trigger keywords via OCR, analyzes screen content and run specified queries.

---

## 📋 Prerequisites

Ensure the following system dependencies are installed (typically on Wayland/Hyprland setups):

- `hyprctl` (Hyprland compositor)
- `grim` (Screenshot utility)
- `tesseract` (OCR engine)
- Rust (edition 2024 / stable)

---

## ⚙️ Environment Variables (`.env`)

Create a `.env` file in the project root based on `.env.example`:

```conf
OPENAI_API_KEY=your_openai_api_key_here
TRIGGER_KEYWORDS=help,error,todo
AI_SYSTEM_PROMPT=""
AI_USER_PROMPT=""
```

---

## 🚀 Building & Running

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

Or via `just`:

```bash
just watch
```

---

## 🧪 Testing

Run the test suite to verify core functionality:

```bash
cargo test
```

---

## 🪟 Hyprland Configuration Example

```conf
# Hyprland binding
bind = $mainMod SHIFT, A, togglespecialworkspace, ai-watcher

windowrule = float on, match:class ^(com\.ai\.floatingterminal)$
windowrule = center on, match:class ^(com\.ai\.floatingterminal)$
windowrule = size 480 640, match:class ^(com\.ai\.floatingterminal)$
windowrule = no_screen_share on, match:class ^(com\.ai\.floatingterminal)$
windowrule = workspace special:ai-watcher silent, match:class ^(com\.ai\.floatingterminal)$
```

---

## ⌨️ TUI Keybindings

- **`j` / `Down`**: Scroll down line-by-line
- **`k` / `Up`**: Scroll up line-by-line
- **`PageDown`**: Scroll down 10 lines
- **`PageUp`**: Scroll up 10 lines
- **`q` / `Esc`**: Quit application

# ✨ PHP Sunshine Features

## 🎨 Material Design Themes

PHP Sunshine includes 5 beautiful Material Design themes:

### 1. Material Ocean (Default) 🌊
Dark blue theme with excellent contrast. Perfect for long coding sessions.
- Background: Deep ocean blue (#0F151C)
- Accent: Teal (#80CBC4)
- Syntax: Vibrant colors optimized for PHP

### 2. Material Palenight 🌙
Purple-tinted dark theme with a modern look.
- Background: Dark purple-blue (#292D3E)
- Accent: Lavender (#C792EA)
- Syntax: Soft, eye-friendly colors

### 3. Material Darker 🖤
Pure dark theme with maximum contrast.
- Background: Almost black (#212121)
- Accent: Teal (#80CBC4)
- Syntax: Bold, vibrant colors

### 4. Material Lighter ☀️
Clean light theme for bright environments.
- Background: Off-white (#FAFAFA)
- Accent: Teal (#00897B)
- Syntax: Dark, readable colors

### 5. Material Deep Ocean 🌑
Darkest theme for OLED screens and low-light coding.
- Background: Pure ocean black (#000B12)
- Accent: Teal (#80CBC4)
- Syntax: Carefully tuned for dark backgrounds

**Switch themes:** `:theme <name>` (ocean, palenight, darker, lighter, deepocean)

## 📁 File Browser

Advanced file tree navigation with smart features:

- **Icon Support**: Material Design file type icons
- **Folder Icons**: Special icons for common folders (src, vendor, tests, etc.)
- **Tree Navigation**: Expand/collapse folders with Enter
- **Smart Sorting**: Directories first, then alphabetical
- **Gitignore Support**: Automatically respects .gitignore files
- **Deep Nesting**: Handles any project structure
- **Visual Depth**: Indented tree view shows hierarchy

### Framework Detection Icons
- 󰐱 PrestaShop modules folder
-  Laravel Artisan
-  Symfony console
- 󰌟 PHP files
-  Template files (Smarty, Blade, Twig)

## 📝 Text Editor

Professional text editing with Rope-based efficiency:

- **Multi-line Editing**: Full support for large files
- **Cursor Management**: Smooth navigation with arrow keys or hjkl
- **Line Numbers**: Always visible with proper formatting
- **Smart Scrolling**: Keeps cursor in view
- **Modified Indicator**: See unsaved changes in tabs
- **File Path Display**: Know which file you're editing
- **Cursor Position**: Line:Column shown in status bar

## 🗂️ Tab System

Efficient multi-file workflow:

- **Multiple Tabs**: Open many files simultaneously
- **Tab Navigation**: Ctrl+Tab / Shift+Tab to switch
- **Modified Indicator**: `*` shows unsaved changes
- **Smart Tab Titles**: Shows filename with modification status
- **Tab Persistence**: Keep working context
- **Close Protection**: Won't close last tab

## 🎯 Framework Detection

Automatically detects your PHP framework:

### PrestaShop 🛒
Detects:
- `config/defines.inc.php`
- `install/` and `classes/` directories
- Module structure
- Hook system files

### Laravel 🔴
Detects:
- `artisan` command file
- `bootstrap/app.php`
- Laravel project structure
- Artisan commands

### Symfony 🎵
Detects:
- `bin/console`
- `symfony.lock`
- Symfony bundles
- Service container

Shows detected framework in header with appropriate icon!

## 🎨 Material Icon Pack

Over 50+ file type icons including:

### Languages
- 󰌟 PHP (.php, .phtml)
-  JavaScript (.js, .jsx, .mjs)
-  TypeScript (.ts, .tsx)
-  HTML (.html, .htm)
-  CSS (.css, .scss, .sass, .less)

### Templates
-  Smarty (.tpl) - PrestaShop
-  Blade (.blade.php) - Laravel
-  Twig (.twig) - Symfony

### Config Files
-  JSON
-  YAML
-  TOML
- 謹 XML
-  INI
-  ENV

### Special Files
-  Composer (composer.json)
-  Package.json
-  Cargo.toml
-  Git files
-  Docker files
-  SQL files
-  Markdown

### Folders
-  Standard folder
-  Open folder
-  Source code (src)
-  Vendor
-  Public/dist
-  Tests
-  Documentation

## ⚡ Performance

Built with Rust for blazing-fast performance:

- **Instant Startup**: Launches in milliseconds
- **Memory Efficient**: Rope data structure for large files
- **60 FPS Rendering**: Smooth terminal UI with Ratatui
- **Async I/O**: Non-blocking file operations with Tokio
- **Tree-sitter Ready**: Prepared for incremental parsing

## 🚀 Coming Soon

### Phase 2 Features (In Progress)
- ✅ PHP Syntax Highlighting with Tree-sitter
- ✅ Code Folding
- ✅ Smart Indentation

### Phase 3 Features
- 📝 Fuzzy File Search (Ctrl+P)
- 🔍 Text Search & Replace
- 📊 Symbol Navigation
- 🎯 Go to Definition

### Phase 4 Features
- 💡 IntelliSense / Code Completion
- 🔧 Refactoring Tools
- 🐛 Integrated Debugger
- 🧪 Test Runner

### Phase 5 Features
- 🔌 LSP (Language Server Protocol)
- 📦 Composer Integration
- 🌳 Git UI (commit, branch, merge)
- 💬 TODO Comments Tracker

## 🎓 Learning Curve

**Easy to learn if you know:**
- Vim keybindings (hjkl navigation, modes)
- Basic terminal usage
- File system navigation

**New to Vim-style editors?**
1. Start with arrow keys in Insert mode
2. Learn `i` (insert), `Esc` (normal), `:w` (save), `:q` (quit)
3. Gradually adopt hjkl navigation
4. Master Ctrl shortcuts for speed

## 💻 System Requirements

- **OS**: macOS, Linux, Windows
- **Terminal**: Any modern terminal (iTerm2, Alacriths, Windows Terminal)
- **Font**: Nerd Font recommended for icons
- **RAM**: < 10MB for typical usage
- **Rust**: 1.70+ (for building from source)

## 🎨 Best Terminal Setup

For the best experience:

1. **Install a Nerd Font**: [Nerd Fonts](https://www.nerdfonts.com/)
   - Recommended: JetBrains Mono Nerd Font, Fira Code Nerd Font
2. **Configure Terminal**: Set font in terminal preferences
3. **True Color Support**: Ensure terminal supports 24-bit color
4. **Test**: Run PHP Sunshine and enjoy the icons! ☀️

## 🌟 Why PHP Sunshine?

- **🦀 Built with Rust**: Memory-safe, blazing fast
- **🎨 Beautiful**: Material Design themes
- **🚀 Lightweight**: No Electron, pure terminal
- **🔧 PHP-Focused**: Made for PrestaShop, Laravel, Symfony
- **📦 Self-contained**: Single binary, no dependencies
- **🎯 Modal Editing**: Efficient Vim-style workflow
- **💼 Professional**: IDE features in terminal

---

Built with ❤️ and Rust for PHP developers! 🦀☀️

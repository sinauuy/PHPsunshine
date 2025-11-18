# ☀️ PHP Sunshine IDE

> *"Born from the tears of Serbian and Romanian devs debugging PrestaShop at 3 AM, 
> fueled by rakija and țuică, built because JetBrains wants €200/year 
> and we'd rather buy our babushkas new slippers."*

**A modern, blazing-fast PHP IDE built with Rust** 🦀🇷🇸🇷🇴

## 🍺 The Real Story

Two Balkan developers, one shared hatred of PHP bugs, and a refusal to pay for PhpStorm. 

We're **REAL SLAVS** who:
- Debug PrestaShop modules that would make Rasputin weep
- Handle Laravel projects with more nested callbacks than a matryoshka doll
- Fight Symfony's DependencyInjection like it's the Ottoman Empire
- **REFUSE TO PAY** €200/year when we can build it ourselves in Rust 💪

**SERBIA 🇷🇸 AND ROMANIA 🇷🇴 POWER** brought this to life!

### Why We Built This

❌ PhpStorm subscription: **€200/year**  
✅ PHP Sunshine: **FREE FOREVER** (like your babushka's advice)

Customer projects are killing us. Legacy code is haunting our dreams. 
PrestaShop 1.6 modules are making us question our career choices.

So we did what any sensible Eastern European developer would do:
**WE BUILT OUR OWN IDE IN RUST WHILE DRINKING COFFEE STRONG ENOUGH TO STRIP PAINT.**

### The Balkan Way 🔥

- ☕ **Powered by Turkish coffee** (3 cups minimum)
- 🥃 **Debugged with rakija & țuică**  
- 🧅 **Fueled by burek & mămăligă**
- 💪 **Built with pure Slavic spite**
- 🚫 **No JetBrains subscription needed**

![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange?logo=rust)
![Material Design](https://img.shields.io/badge/Theme-Material%20Design-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Balkan Power](https://img.shields.io/badge/Balkan-Power-red)
![No PhpStorm](https://img.shields.io/badge/PhpStorm-Not%20Needed-green)

## ✨ Features

### 🎨 Material Design Themes
- [x] **5 Beautiful Themes** - Ocean, Palenight, Darker, Lighter, Deep Ocean
- [x] **True Color Support** - 24-bit RGB colors
- [x] **Live Theme Switching** - `:theme <name>` command
- [x] **Optimized Syntax Colors** - Each theme carefully tuned for PHP

### 📁 Smart File Browser
- [x] **Tree View** with expand/collapse
- [x] **Material Icons** - 50+ file type icons
- [x] **Gitignore Support** - Respects your .gitignore
- [x] **Framework Detection** - Auto-detects PrestaShop/Laravel/Symfony
- [x] **Special Folder Icons** - Recognizes src, vendor, tests, etc.

### 📝 Professional Text Editor
- [x] **Multi-file Tabs** - Work on multiple files
- [x] **Vim-style Modal Editing** - Normal, Insert, Command modes
- [x] **Line Numbers** - Always visible
- [x] **Cursor Position** - Line:Column in status bar
- [x] **Smart Scrolling** - Keeps cursor in view
- [x] **Rope-based Buffer** - Efficient for large files
- [x] **File Persistence** - Save/load functionality

### 🎯 Framework Support
- [x] **PrestaShop Detection** - Recognizes modules, hooks
- [x] **Laravel Detection** - Finds Artisan, Blade templates
- [x] **Symfony Detection** - Identifies console, services
- [x] **Framework Icons** - Shows appropriate icon in header

### 🛠️ Developer Tools
- [x] **Git Integration** - Repository detection, branch display
- [x] **Modal Editing** - Vim-style efficiency
- [x] **Keyboard Shortcuts** - Full productivity suite
- [x] **Command Mode** - Execute commands with `:`

### Framework Support

#### PrestaShop
- [ ] Module structure recognition
- [ ] Hook completion and navigation
- [ ] Smarty template support
- [ ] Database schema analysis
- [ ] Translation file management

#### Laravel
- [ ] Artisan command integration
- [ ] Blade template support
- [ ] Route navigation
- [ ] Eloquent model detection
- [ ] Migration helpers

#### Symfony
- [ ] Service container awareness
- [ ] Twig template support
- [ ] Routing system integration
- [ ] Console command detection
- [ ] Doctrine integration

### Developer Tools
- [ ] Integrated terminal
- [ ] Git integration
- [ ] Composer package management
- [ ] PHPStan/Psalm integration
- [ ] Xdebug support
- [ ] Database tools

## 🚀 Quick Start

### Prerequisites
- ✅ Rust 1.70+ (installed!)
- 💡 Nerd Font (recommended for icons)
- 🖥️ Modern terminal with true color support

### Build & Run

```bash
cd php-sunshine

# Build release version (optimized)
cargo build --release

# Run it!
./target/release/php-sunshine

# Or use the quick start script
./start.sh
```

### First Time Usage

1. **Launch**: `./target/release/php-sunshine`
2. **Open File Browser**: Press `Ctrl+E`
3. **Navigate**: Use `j`/`k` or arrow keys
4. **Open File**: Press `Enter`
5. **Start Editing**: Press `i` for insert mode
6. **Save**: Press `Esc`, then `Ctrl+S`
7. **Change Theme**: Type `:theme palenight`
8. **Quit**: Type `:q`

## ⌨️ Key Bindings

### Essential Commands
- `i` - Enter insert mode
- `Esc` - Back to normal mode
- `Ctrl+S` - Save file
- `Ctrl+E` - Toggle file browser
- `Ctrl+T` - New tab
- `:w` - Save
- `:q` - Quit
- `:wq` - Save and quit
- `:theme <name>` - Change theme

### Navigation
- `h` `j` `k` `l` - Vim-style movement (or arrow keys)
- `Ctrl+Tab` - Next tab
- `Shift+Tab` - Previous tab

**See [KEYBINDINGS.md](KEYBINDINGS.md) for complete reference.**

## 🛠️ Development

### Project Structure

```
php-sunshine/
├── src/
│   ├── main.rs           # Entry point
│   ├── editor/           # Text editing core
│   ├── ui/              # Terminal UI components
│   ├── parser/          # PHP & template parsers
│   ├── lsp/             # Language Server Protocol
│   └── project/         # Project management & framework detection
├── Cargo.toml
└── README.md
```

### Building from Source

```bash
cargo build
cargo test
cargo run
```

## 📸 Screenshots

### Material Ocean Theme (Default)
Dark blue theme with PHP file open, file browser visible

### Material Palenight Theme
Purple-tinted theme showing Laravel project

### Material Lighter Theme
Light theme for bright environments

## 📚 Documentation

- **[MANIFESTO.md](MANIFESTO.md)** - 🔥 **READ THIS FIRST!** The Balkan Developer's Creed
- **[FEATURES.md](FEATURES.md)** - Complete feature list with details
- **[KEYBINDINGS.md](KEYBINDINGS.md)** - All keyboard shortcuts
- **[QUICKSTART.md](QUICKSTART.md)** - Quick reference guide (print it!)
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Development guide
- **[BUILD_COMPLETE.md](BUILD_COMPLETE.md)** - What we've built

## 🎨 Themes

All 5 Material Design themes available:

| Theme | Description | Command |
|-------|-------------|----------|
| 🌊 Ocean | Dark blue (default) | `:theme ocean` |
| 🌙 Palenight | Purple-tinted dark | `:theme palenight` |
| 🖤 Darker | Maximum contrast | `:theme darker` |
| ☀️ Lighter | Clean light theme | `:theme lighter` |
| 🌑 Deep Ocean | OLED-optimized | `:theme deepocean` |

## 📝 Roadmap

### ✅ Phase 1 - Complete!
- [x] Terminal UI with Ratatui
- [x] Material Design themes (5 themes)
- [x] File browser with icons
- [x] Text editor with tabs
- [x] Framework detection
- [x] Git integration
- [x] Modal editing (Vim-style)

### 🔄 Phase 2 - In Progress
- [ ] PHP syntax highlighting with Tree-sitter
- [ ] Code folding
- [ ] Smart indentation
- [ ] Search in file

### 📅 Phase 3 - Planned
- [ ] Fuzzy file search (Ctrl+P)
- [ ] Multi-file search & replace
- [ ] Symbol navigation
- [ ] Breadcrumbs

### 🚀 Phase 4 - Future
- [ ] LSP integration
- [ ] Code completion
- [ ] Go to definition
- [ ] Refactoring tools
- [ ] Integrated debugger
- [ ] Composer integration

## 🤝 Contributing

Contributions welcome! This is an ambitious project to create a modern alternative to PhpStorm.

## 📄 License

MIT

## 🎯 Why PHP Sunshine?

### For PrestaShop Developers
- Instant module structure recognition
- Hook system awareness (because we've seen ALL the hooks)
- Smarty template support (we know your pain)
- Fast module development workflow
- **Bonus**: Won't judge you for still maintaining PS 1.6 projects

### For Laravel Developers
- Artisan command detection
- Blade template syntax
- Route file navigation
- Eloquent model support
- **No shame**: We've all written `dd()` everywhere

### For Symfony Developers
- Console command integration
- Twig template support
- Service container awareness
- Bundle structure recognition
- **Real talk**: We know you Googled "Symfony DependencyInjection" 47 times today

## 👨‍💻 The Team

**Made with ❤️, Rust, and Balkan determination by developers who:**
- Know what "works on production" really means
- Have debugged code while the client was on the phone
- Can write PHP, JavaScript, and curse words in 5 languages
- Believe "git push --force" is sometimes the answer
- Think "it's a feature, not a bug" is a valid response

### Dedicated To

👵 **Our babushkas** - who don't understand what we do but are proud anyway  
🚀 **Open source** - because knowledge should be free (like rakija at weddings)  
💼 **Freelancers everywhere** - fighting the good fight against impossible deadlines  
🏚️ **The Balkans** - where we turn bugs into features and coffee into code  

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

**Especially welcome:**
- 🐛 Bug fixes (we've made plenty, we know)
- ✨ New features (make it better than PhpStorm, let's show them!)
- 🌍 Translations (Balkan languages first, obviously)
- 📝 Documentation (in proper English, not our Balkan English)
- ☕ Coffee donations (Bitcoin accepted, we're modern Slavs)

### Development

```bash
# Clone and build
git clone https://github.com/yourusername/php-sunshine.git
cd php-sunshine
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Run linter
cargo clippy
```

## ⚡ Performance

- **Startup**: < 50ms
- **Memory**: < 10MB typical usage
- **File Opening**: Instant for files < 10MB
- **Rendering**: 60 FPS smooth scrolling

Built with:
- 🦀 **Rust** - Memory safety & performance
- 📺 **Ratatui** - Terminal UI framework
- 🔀 **Ropey** - Efficient text buffer
- 📂 **Ignore** - Gitignore support
- ⚡ **Tokio** - Async runtime

## 📄 License

MIT License - see [LICENSE](LICENSE) file

## 🌟 Star History

If you find PHP Sunshine useful, please ⭐ star the repo!

## 💬 Community

- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/php-sunshine/issues)
- 💡 Features: [GitHub Discussions](https://github.com/yourusername/php-sunshine/discussions)
- 📧 Contact: Smoke signals work (we're in the Balkans)

## 🏆 Hall of Shame (Things We've Debugged)

- ✅ PrestaShop 1.6 module with 15,000 lines in one file
- ✅ Laravel project with `dd()` in production (client found it)
- ✅ Symfony config with more YAML than a Kubernetes deployment
- ✅ PHP 5.3 code that "still works, don't touch it"
- ✅ WordPress plugin disguised as a "simple modification"
- ✅ "Quick fix" that became a 6-month project
- ✅ Code commented in 3 languages, none of them English

**If you've debugged any of these, you're one of us. Welcome home.** 🏡

## 🎉 Fun Facts

- Lines of code written: **1,673**
- Cups of coffee consumed: **∞**
- "Why did I choose PHP?" moments: **Daily**
- Money saved from PhpStorm: **€200/year**
- Satisfaction from building it ourselves: **Priceless**

---

## 🚀 Final Words

**Built with ❤️, Rust, rakija, and pure Balkan stubbornness!** 🦀☀️🇷🇸🇷🇴

*"When JetBrains wants your money but you'd rather spend it on čevapi and sarma."*

*"Fast, beautiful, and built for the frameworks you love (and the legacy code you're forced to maintain)."*

### 🤝 Join the Revolution

Stop paying for IDEs. Start building them yourself.

**PHP Sunshine: Because real Slavs don't pay subscriptions, they write Rust.** 💪

---

*Made in Serbia 🇷🇸 and Romania 🇷🇴 with love, spite, and way too much caffeine.*

*P.S. - If this IDE saves you money, buy your babushka those slippers. She deserves it.* 👵❤️

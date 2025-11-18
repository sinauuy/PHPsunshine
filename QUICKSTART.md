# ⚡ PHP Sunshine - Quick Start Guide

## 🚀 Launch

```bash
cd /Users/nikola/Sites/php-sunshine
./target/release/php-sunshine
```

## 📋 Essential Shortcuts (Print This!)

### 🎯 Most Used Commands
```
Ctrl+E    Toggle file browser
Ctrl+S    Save file
Ctrl+T    New tab
Ctrl+Tab  Next tab
Ctrl+C    Quit
```

### ⌨️ Modes
```
i         Enter INSERT mode
Esc       Return to NORMAL mode
:         Enter COMMAND mode
```

### 🎨 Themes
```
:theme ocean      (default) Dark blue
:theme palenight  Purple-tinted dark
:theme darker     Maximum contrast
:theme lighter    Clean light
:theme deepocean  OLED optimized
```

### 📁 File Operations
```
:w        Save
:q        Quit
:wq       Save and quit
```

### 🧭 Navigation
```
h j k l   Vim-style movement
←↓↑→      Arrow keys work too!
```

## 🎓 First Session Tutorial

### Step 1: Open File Browser
1. Launch: `./target/release/php-sunshine`
2. Press `Ctrl+E` (you'll see the file tree on the left)

### Step 2: Navigate & Open
1. Use `j` or `↓` to move down
2. Use `k` or `↑` to move up
3. Press `Enter` on a folder to expand
4. Press `Enter` on a file to open

### Step 3: Edit
1. Press `i` to start editing (INSERT mode)
2. Type your PHP code
3. Press `Esc` when done (NORMAL mode)

### Step 4: Save
1. Press `Ctrl+S` (or type `:w`)
2. You'll see "File saved" in status bar

### Step 5: Multiple Files
1. Press `Ctrl+E` to open file browser again
2. Open another file
3. See both tabs at top
4. Press `Ctrl+Tab` to switch

### Step 6: Change Theme
1. Press `:` (colon)
2. Type `theme palenight`
3. Press `Enter`
4. Watch the colors change!

### Step 7: Quit
1. Make sure you're in NORMAL mode (press `Esc`)
2. Type `:q`
3. Press `Enter`

## 💡 Pro Tips

### Tip #1: File Browser is Smart
- It shows icons for different file types
- Folders like `vendor`, `src`, `tests` get special icons
- It respects `.gitignore` automatically

### Tip #2: Status Bar Info
Look at the bottom:
- Left: Current mode (NORMAL/INSERT/COMMAND)
- Middle: Status message
- Right: Cursor position (line:column)

### Tip #3: Modified Files
Tabs show `*` when file has unsaved changes:
- `index.php` - Saved
- `index.php*` - Modified

### Tip #4: Framework Detection
Header shows what framework you're in:
- 󰐱 PrestaShop (shopping cart icon)
-  Laravel (Laravel icon)
-  Symfony (Symfony icon)
- 󰌟 Generic PHP

### Tip #5: Vim Users
If you know Vim, you're already 80% there!
- Same hjkl navigation
- Same modes (Normal/Insert/Command)
- Same :w, :q, :wq commands

### Tip #6: New to Vim?
Start with arrow keys! Learn gradually:
1. Week 1: Use arrow keys, learn i/Esc/:w/:q
2. Week 2: Try hjkl instead of arrows
3. Week 3: Master Ctrl shortcuts
4. Week 4: You're a pro! ☀️

## 🎨 Theme Showcase

### Material Ocean (Default)
```
:theme ocean
```
- Deep blue background
- Teal accents
- Perfect for long coding sessions
- Easy on the eyes

### Material Palenight
```
:theme palenight
```
- Purple-tinted dark
- Lavender accents
- Modern & stylish
- Great for evening coding

### Material Darker
```
:theme darker
```
- Almost black background
- Maximum contrast
- Bold colors
- For focused work

### Material Lighter
```
:theme lighter
```
- Clean white/light gray
- Dark text
- For bright environments
- Daytime coding

### Material Deep Ocean
```
:theme deepocean
```
- Darkest available
- Pure black
- OLED friendly
- Low-light coding

## 🎯 Common Workflows

### Edit Existing PHP File
```
1. Ctrl+E → Open browser
2. Navigate to file → Enter
3. i → Start editing
4. Esc → Stop editing
5. Ctrl+S → Save
```

### Create New File (Workaround)
```
1. Use external terminal to create file
2. Refresh by toggling browser (Ctrl+E twice)
3. Open the new file
```

### Work on Multiple Files
```
1. Open first file
2. Ctrl+E → Open browser
3. Open second file
4. Ctrl+Tab to switch between them
5. Ctrl+S on each to save
```

### Quick Edit & Quit
```
1. Open file
2. i → Make changes
3. Esc
4. :wq → Save and quit
```

## 🐛 Troubleshooting

### Icons Look Weird?
→ Install a Nerd Font:
```bash
brew tap homebrew/cask-fonts
brew install --cask font-jetbrains-mono-nerd-font
```
Then set it in your terminal preferences.

### Colors Look Wrong?
→ Make sure your terminal supports true color (24-bit).
Most modern terminals do: iTerm2, Alacritty, Windows Terminal.

### Can't See File Browser?
→ Press `Ctrl+E` to toggle it on.

### Stuck in Insert Mode?
→ Press `Esc` to get back to Normal mode.

### How to Undo?
→ Not implemented yet! Use Ctrl+Z in your terminal
   or save often with Ctrl+S.

## 📊 Feature Checklist

When learning, check off as you master each:

- [ ] Launch PHP Sunshine
- [ ] Toggle file browser (Ctrl+E)
- [ ] Navigate with j/k or arrows
- [ ] Open a file (Enter)
- [ ] Enter insert mode (i)
- [ ] Edit text
- [ ] Return to normal mode (Esc)
- [ ] Save file (Ctrl+S)
- [ ] Open multiple files
- [ ] Switch tabs (Ctrl+Tab)
- [ ] Change theme (:theme)
- [ ] Quit (:q)

## 🎊 You're Ready!

Once you've completed the checklist above, you know:
- ✅ File navigation
- ✅ Text editing
- ✅ File management
- ✅ Multi-file workflow
- ✅ Theme customization

**You're now a PHP Sunshine user! ☀️**

---

## 🔗 More Resources

- **Full Features**: See [FEATURES.md](FEATURES.md)
- **All Shortcuts**: See [KEYBINDINGS.md](KEYBINDINGS.md)
- **Contributing**: See [CONTRIBUTING.md](CONTRIBUTING.md)
- **Build Details**: See [BUILD_COMPLETE.md](BUILD_COMPLETE.md)

---

**Made with ❤️ and Rust for PHP developers! 🦀☀️**

*Print this page and keep it next to your keyboard!*

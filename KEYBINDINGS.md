# ⌨️ PHP Sunshine Keybindings

## Modes

PHP Sunshine has three modes (similar to Vim):
- **Normal Mode**: Navigate and execute commands
- **Insert Mode**: Edit text
- **Command Mode**: Enter commands with `:`

## Normal Mode

### File Operations
- `Ctrl+S` - Save current file
- `Ctrl+T` - New tab
- `Ctrl+W` - Close current tab
- `Ctrl+Tab` - Next tab
- `Shift+Tab` - Previous tab

### Navigation
- `h` / `←` - Move cursor left
- `j` / `↓` - Move cursor down
- `k` / `↑` - Move cursor up
- `l` / `→` - Move cursor right

### File Browser
- `Ctrl+E` - Toggle file browser
- `j` / `↓` - Move down in file list
- `k` / `↑` - Move up in file list
- `Enter` - Open file or toggle folder

### Mode Switching
- `i` - Enter Insert mode
- `:` - Enter Command mode
- `q` - Quit (in Normal mode)
- `Ctrl+C` - Force quit

## Insert Mode

### Editing
- All regular typing
- `Enter` - New line
- `Backspace` - Delete previous character
- `Delete` - Delete current character
- `Arrow keys` - Navigate cursor

### Exit Insert Mode
- `Esc` - Return to Normal mode

## Command Mode

Commands are entered by pressing `:` in Normal mode, then typing:

### File Commands
- `:w` or `:write` - Save current file
- `:q` or `:quit` - Quit
- `:wq` - Save and quit

### Theme Commands
- `:theme <name>` - Change theme
  - `:theme ocean` - Material Ocean (default)
  - `:theme palenight` - Material Palenight
  - `:theme darker` - Material Darker
  - `:theme lighter` - Material Lighter (light theme)
  - `:theme deepocean` - Material Deep Ocean

### Navigation
- `Esc` - Cancel and return to Normal mode

## Tips

1. **Quick Save**: Press `Ctrl+S` in Normal mode
2. **Fast File Switching**: Use `Ctrl+Tab` to cycle through open files
3. **Quick Theme Change**: Try `:theme palenight` for a purple theme
4. **File Browser**: Press `Ctrl+E` to toggle, navigate with j/k, open with Enter
5. **Modified Files**: Tabs show `*` for unsaved changes

## Special Characters in Display

- ☀️ - PHP Sunshine logo
- 󰌟 - PHP file
-  - JavaScript file
-  - HTML file
-  - Folder (closed)
-  - Folder (open)
-  - Git icon
-  - Laravel icon
-  - Symfony icon
- 󰐱 - PrestaShop (shopping cart) icon

## Quick Start Example

1. Launch PHP Sunshine: `./php-sunshine`
2. Press `Ctrl+E` to open file browser
3. Navigate with `j`/`k` and press `Enter` on a file
4. Press `i` to start editing
5. Edit your file
6. Press `Esc` to exit insert mode
7. Press `Ctrl+S` to save
8. Try `:theme darker` to change the theme
9. Press `:q` to quit

/// Text buffer management using Ropey
/// This module handles efficient text editing operations

use ropey::Rope;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn char_index(&self, rope: &Rope) -> usize {
        let line_idx = self.line.min(rope.len_lines().saturating_sub(1));
        let line_start = rope.line_to_char(line_idx);
        let line = rope.line(line_idx);
        let col = self.column.min(line.len_chars().saturating_sub(1));
        line_start + col
    }
}

pub struct TextBuffer {
    rope: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
    cursor: Cursor,
    scroll_offset: usize,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
        }
    }

    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
            file_path: None,
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self {
            rope: Rope::from_str(&content),
            file_path: Some(path.to_path_buf()),
            modified: false,
            cursor: Cursor::new(),
            scroll_offset: 0,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = self.file_path.clone() {
            self.save_to(&path)?;
        }
        Ok(())
    }

    pub fn save_to(&mut self, path: &Path) -> Result<()> {
        let content = self.rope.to_string();
        fs::write(path, content)?;
        self.file_path = Some(path.to_path_buf());
        self.modified = false;
        Ok(())
    }

    pub fn insert_char(&mut self, ch: char) {
        let pos = self.cursor.char_index(&self.rope);
        self.rope.insert_char(pos, ch);
        self.cursor.column += 1;
        self.modified = true;
    }

    pub fn insert(&mut self, text: &str) {
        let pos = self.cursor.char_index(&self.rope);
        self.rope.insert(pos, text);
        self.modified = true;
    }

    pub fn delete_char(&mut self) {
        let pos = self.cursor.char_index(&self.rope);
        if pos < self.rope.len_chars() {
            self.rope.remove(pos..pos + 1);
            self.modified = true;
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
            self.delete_char();
        } else if self.cursor.line > 0 {
            // Move to end of previous line
            self.cursor.line -= 1;
            let line = self.rope.line(self.cursor.line);
            self.cursor.column = line.len_chars().saturating_sub(1);
            self.delete_char();
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let line = self.rope.line(self.cursor.line);
            self.cursor.column = self.cursor.column.min(line.len_chars().saturating_sub(1));
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor.line < self.rope.len_lines().saturating_sub(1) {
            self.cursor.line += 1;
            let line = self.rope.line(self.cursor.line);
            self.cursor.column = self.cursor.column.min(line.len_chars().saturating_sub(1));
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let line = self.rope.line(self.cursor.line);
            self.cursor.column = line.len_chars().saturating_sub(1);
        }
    }

    pub fn move_cursor_right(&mut self) {
        let line = self.rope.line(self.cursor.line);
        if self.cursor.column < line.len_chars().saturating_sub(1) {
            self.cursor.column += 1;
        } else if self.cursor.line < self.rope.len_lines().saturating_sub(1) {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn len(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, idx: usize) -> String {
        if idx < self.rope.len_lines() {
            self.rope.line(idx).to_string()
        } else {
            String::new()
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    pub fn file_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "[No Name]".to_string())
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn update_scroll(&mut self, viewport_height: usize) {
        // Keep cursor in view
        if self.cursor.line < self.scroll_offset {
            self.scroll_offset = self.cursor.line;
        } else if self.cursor.line >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.cursor.line - viewport_height + 1;
        }
    }
}

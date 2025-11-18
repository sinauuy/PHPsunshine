/// Tab system for managing multiple open files
/// Supports switching between files and tracking modifications

use crate::editor::TextBuffer;
use std::path::Path;
use anyhow::Result;

pub struct Tab {
    pub buffer: TextBuffer,
    pub id: usize,
}

impl Tab {
    pub fn new(id: usize, buffer: TextBuffer) -> Self {
        Self { buffer, id }
    }

    pub fn title(&self) -> String {
        let modified = if self.buffer.is_modified() { "*" } else { "" };
        format!("{}{}", self.buffer.file_name(), modified)
    }
}

pub struct TabManager {
    tabs: Vec<Tab>,
    active_index: usize,
    next_id: usize,
}

impl TabManager {
    pub fn new() -> Self {
        let mut tabs = Vec::new();
        tabs.push(Tab::new(0, TextBuffer::new()));
        
        Self {
            tabs,
            active_index: 0,
            next_id: 1,
        }
    }

    pub fn open_file(&mut self, path: &Path) -> Result<()> {
        // Check if already open
        for (idx, tab) in self.tabs.iter().enumerate() {
            if tab.buffer.file_path() == Some(path) {
                self.active_index = idx;
                return Ok(());
            }
        }

        // Open new tab
        let buffer = TextBuffer::from_file(path)?;
        let tab = Tab::new(self.next_id, buffer);
        self.next_id += 1;
        
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        
        Ok(())
    }

    pub fn new_tab(&mut self) {
        let tab = Tab::new(self.next_id, TextBuffer::new());
        self.next_id += 1;
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
    }

    pub fn close_current(&mut self) -> bool {
        if self.tabs.len() <= 1 {
            return false;
        }

        self.tabs.remove(self.active_index);
        if self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len().saturating_sub(1);
        }
        
        true
    }

    pub fn next_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_index = (self.active_index + 1) % self.tabs.len();
        }
    }

    pub fn prev_tab(&mut self) {
        if self.tabs.len() > 1 {
            self.active_index = if self.active_index == 0 {
                self.tabs.len() - 1
            } else {
                self.active_index - 1
            };
        }
    }

    pub fn active_tab(&mut self) -> &mut Tab {
        &mut self.tabs[self.active_index]
    }

    pub fn active_buffer(&mut self) -> &mut TextBuffer {
        &mut self.tabs[self.active_index].buffer
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn has_modified(&self) -> bool {
        self.tabs.iter().any(|t| t.buffer.is_modified())
    }

    pub fn save_current(&mut self) -> Result<()> {
        self.tabs[self.active_index].buffer.save()
    }

    pub fn save_all(&mut self) -> Result<()> {
        for tab in &mut self.tabs {
            if tab.buffer.is_modified() {
                tab.buffer.save()?;
            }
        }
        Ok(())
    }
}

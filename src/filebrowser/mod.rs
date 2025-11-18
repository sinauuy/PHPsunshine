/// File browser with tree view and icon support
/// Respects .gitignore and provides filtering

use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use ignore::WalkBuilder;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub depth: usize,
    pub children: Vec<FileEntry>,
}

impl FileEntry {
    pub fn new(path: PathBuf, depth: usize) -> Result<Self> {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        let is_dir = path.is_dir();
        
        Ok(Self {
            path,
            name,
            is_dir,
            is_expanded: false,
            depth,
            children: Vec::new(),
        })
    }

    pub fn load_children(&mut self) -> Result<()> {
        if !self.is_dir || self.is_expanded {
            return Ok(());
        }

        self.children.clear();
        
        if let Ok(entries) = fs::read_dir(&self.path) {
            let mut children: Vec<FileEntry> = entries
                .filter_map(|e| e.ok())
                .filter_map(|e| FileEntry::new(e.path(), self.depth + 1).ok())
                .collect();
            
            // Sort: directories first, then alphabetically
            children.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });
            
            self.children = children;
        }
        
        self.is_expanded = true;
        Ok(())
    }

    pub fn toggle(&mut self) -> Result<()> {
        if !self.is_dir {
            return Ok(());
        }

        if self.is_expanded {
            self.is_expanded = false;
            self.children.clear();
        } else {
            self.load_children()?;
        }
        
        Ok(())
    }

    pub fn flatten(&self) -> Vec<&FileEntry> {
        let mut result = vec![self];
        
        if self.is_expanded {
            for child in &self.children {
                result.extend(child.flatten());
            }
        }
        
        result
    }
}

pub struct FileBrowser {
    root: FileEntry,
    selected_index: usize,
    scroll_offset: usize,
}

impl FileBrowser {
    pub fn new(root_path: &Path) -> Result<Self> {
        let mut root = FileEntry::new(root_path.to_path_buf(), 0)?;
        root.load_children()?;
        
        Ok(Self {
            root,
            selected_index: 0,
            scroll_offset: 0,
        })
    }

    pub fn get_visible_entries(&self) -> Vec<&FileEntry> {
        self.root.flatten()
    }

    pub fn selected_entry(&self) -> Option<&FileEntry> {
        let entries = self.get_visible_entries();
        entries.get(self.selected_index).copied()
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        let entries = self.get_visible_entries();
        if self.selected_index < entries.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn toggle_selected(&mut self) -> Result<()> {
        let path = {
            let entries = self.get_visible_entries();
            entries.get(self.selected_index).map(|e| e.path.clone())
        };
        
        if let Some(path) = path {
            self.toggle_entry(&path)?;
        }
        Ok(())
    }

    fn toggle_entry(&mut self, path: &Path) -> Result<()> {
        Self::toggle_recursive_static(&mut self.root, path)
    }

    fn toggle_recursive_static(entry: &mut FileEntry, target: &Path) -> Result<()> {
        if entry.path == target {
            entry.toggle()?;
            return Ok(());
        }

        for child in &mut entry.children {
            if target.starts_with(&child.path) {
                Self::toggle_recursive_static(child, target)?;
                return Ok(());
            }
        }

        Ok(())
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn update_scroll(&mut self, viewport_height: usize) {
        let visible_count = self.get_visible_entries().len();
        
        // Keep selected item in view
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.selected_index - viewport_height + 1;
        }
        
        // Don't scroll past the end
        if self.scroll_offset + viewport_height > visible_count {
            self.scroll_offset = visible_count.saturating_sub(viewport_height);
        }
    }

    pub fn search_files(&self, query: &str) -> Vec<PathBuf> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        self.search_recursive(&self.root, &query_lower, &mut results);
        
        results
    }

    fn search_recursive(&self, entry: &FileEntry, query: &str, results: &mut Vec<PathBuf>) {
        if entry.name.to_lowercase().contains(query) {
            results.push(entry.path.clone());
        }
        
        for child in &entry.children {
            self.search_recursive(child, query, results);
        }
    }
}

/// Walk directory respecting .gitignore
pub fn walk_directory(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for result in WalkBuilder::new(root)
        .hidden(false)
        .git_ignore(true)
        .build()
    {
        if let Ok(entry) = result {
            files.push(entry.path().to_path_buf());
        }
    }
    
    Ok(files)
}

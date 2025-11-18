/// Material Design icon pack for file types and UI elements
/// Uses Unicode symbols and Nerd Font icons

use std::path::Path;

pub struct MaterialIcons;

impl MaterialIcons {
    /// Get icon for file based on extension
    pub fn for_file(path: &Path) -> &'static str {
        if path.is_dir() {
            return Self::folder();
        }

        match path.extension().and_then(|s| s.to_str()) {
            // PHP files
            Some("php") => "󰌟",  // PHP icon
            Some("phtml") => "󰌟",
            
            // JavaScript/TypeScript
            Some("js") => "",
            Some("jsx") => "",
            Some("ts") => "",
            Some("tsx") => "",
            Some("mjs") => "",
            
            // Web files
            Some("html") => "",
            Some("htm") => "",
            Some("css") => "",
            Some("scss") => "",
            Some("sass") => "",
            Some("less") => "",
            
            // Templates
            Some("tpl") => "",  // Smarty (PrestaShop)
            Some("blade.php") => "",  // Laravel Blade
            Some("twig") => "",  // Symfony Twig
            
            // Config files
            Some("json") => "",
            Some("yaml") | Some("yml") => "",
            Some("toml") => "",
            Some("xml") => "謹",
            Some("ini") => "",
            Some("env") => "",
            
            // PHP frameworks
            Some("lock") if path.file_name().unwrap_or_default() == "composer.lock" => "",
            
            // Database
            Some("sql") => "",
            Some("db") | Some("sqlite") => "",
            
            // Documentation
            Some("md") => "",
            Some("txt") => "",
            Some("pdf") => "",
            
            // Images
            Some("png") | Some("jpg") | Some("jpeg") | Some("gif") | 
            Some("svg") | Some("ico") | Some("webp") => "",
            
            // Git
            _ if path.file_name().unwrap_or_default() == ".gitignore" => "",
            _ if path.file_name().unwrap_or_default() == ".gitattributes" => "",
            
            // Docker
            _ if path.to_str().unwrap_or("").contains("Dockerfile") => "",
            _ if path.file_name().unwrap_or_default() == "docker-compose.yml" => "",
            
            // Build/Package files
            _ if path.file_name().unwrap_or_default() == "composer.json" => "",
            _ if path.file_name().unwrap_or_default() == "package.json" => "",
            _ if path.file_name().unwrap_or_default() == "Cargo.toml" => "",
            
            // Default
            _ => "",
        }
    }

    /// Get icon for special folders
    pub fn folder() -> &'static str {
        ""
    }

    pub fn folder_open() -> &'static str {
        ""
    }

    pub fn folder_special(name: &str) -> &'static str {
        match name {
            "src" | "source" => "",
            "vendor" => "",
            "node_modules" => "",
            "public" | "dist" | "build" => "",
            "test" | "tests" | "__tests__" => "",
            "docs" | "documentation" => "",
            ".git" => "",
            "config" => "",
            "assets" | "images" | "img" => "",
            "modules" => "󰏗",  // PrestaShop modules
            "themes" => "",
            "controllers" => "",
            "models" => "",
            "views" => "",
            _ => Self::folder(),
        }
    }

    // UI Icons
    pub fn git_branch() -> &'static str {
        ""
    }

    pub fn git_added() -> &'static str {
        "✚"
    }

    pub fn git_modified() -> &'static str {
        "●"
    }

    pub fn git_deleted() -> &'static str {
        "✖"
    }

    pub fn search() -> &'static str {
        ""
    }

    pub fn settings() -> &'static str {
        ""
    }

    pub fn terminal() -> &'static str {
        ""
    }

    pub fn close() -> &'static str {
        ""
    }

    pub fn save() -> &'static str {
        ""
    }

    pub fn error() -> &'static str {
        ""
    }

    pub fn warning() -> &'static str {
        ""
    }

    pub fn info() -> &'static str {
        ""
    }

    pub fn success() -> &'static str {
        ""
    }

    // Framework-specific icons
    pub fn laravel() -> &'static str {
        ""
    }

    pub fn symfony() -> &'static str {
        ""
    }

    pub fn prestashop() -> &'static str {
        "󰐱"  // Shopping cart for PrestaShop
    }
}

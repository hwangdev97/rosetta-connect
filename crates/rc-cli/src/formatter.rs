use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use comfy_table::{Table, presets::UTF8_FULL, ContentArrangement, Cell};
use owo_colors::OwoColorize;
use std::collections::BTreeMap;

pub struct DataFormatter;

impl DataFormatter {
    /// Format app metadata in a structured, readable way
    pub fn format_app_metadata(data: &Value) -> String {
        let mut output = String::new();
        
        if let Some(app_id) = data.get("appId").and_then(|v| v.as_str()) {
            output.push_str(&format!("📱 App: {}\n", app_id));
        }
        
        if let Some(locales) = data.get("locales").and_then(|v| v.as_array()) {
            let locale_list: Vec<String> = locales
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
            output.push_str(&format!("🌍 Available Locales: {}\n\n", locale_list.join(", ")));
        }
        
        if let Some(metadata) = data.get("metadata").and_then(|v| v.as_object()) {
            // Get default locale from the data, fallback to en-US
            let default_locale = data.get("defaultLocale")
                .and_then(|v| v.as_str())
                .unwrap_or("en-US");
                
            // Sort locales, but prioritize default locale first
            let mut sorted_locales: Vec<String> = metadata.keys().cloned().collect();
            sorted_locales.sort();
            
            // Move default locale to front if it exists
            if let Some(pos) = sorted_locales.iter().position(|x| x == default_locale) {
                let default = sorted_locales.remove(pos);
                sorted_locales.insert(0, default);
            }
            
            for locale in sorted_locales {
                if let Some(locale_data) = metadata.get(&locale) {
                    output.push_str(&Self::format_locale_metadata(&locale, locale_data));
                    output.push('\n');
                }
            }
        }
        
        output
    }
    
    /// Format metadata for a specific locale
    pub fn format_locale_metadata(locale: &str, data: &Value) -> String {
        let mut output = String::new();
        let locale_flag = Self::get_locale_flag(locale);
        
        output.push_str(&format!("{} {}\n", locale_flag, locale));
        output.push_str(&"─".repeat(40));
        output.push('\n');
        
        if let Some(obj) = data.as_object() {
            // Define the order we want to display fields
            let field_order = ["name", "description", "keywords", "whatsNew"];
            
            for field in &field_order {
                if let Some(value) = obj.get(*field).and_then(|v| v.as_str()) {
                    output.push_str(&Self::format_metadata_field(field, value));
                }
            }
            
            // Display any other fields not in the standard order
            for (key, value) in obj {
                if !field_order.contains(&key.as_str()) {
                    if let Some(text) = value.as_str() {
                        output.push_str(&Self::format_metadata_field(key, text));
                    }
                }
            }
        }
        
        output
    }

    /// Format only the default locale's metadata
    pub fn format_default_locale(data: &Value) -> String {
        let default_locale = data
            .get("defaultLocale")
            .and_then(|v| v.as_str())
            .unwrap_or("en-US");
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec![
            Cell::new("Field").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Value").add_attribute(comfy_table::Attribute::Bold),
        ]);

        let mut add_row = |k: &str, v: Option<&str>| {
            let key = k.blue().to_string();
            let val = v.unwrap_or("");
            let styled_val = if val.is_empty() { "—".dimmed().to_string() } else { val.green().to_string() };
            table.add_row(vec![key, styled_val]);
        };

        if let Some(meta) = data.get("metadata").and_then(|m| m.get(default_locale)) {
            add_row("Locale", Some(default_locale));
            add_row("App Name", meta.get("name").and_then(|v| v.as_str()));
            add_row("Subtitle", meta.get("subtitle").and_then(|v| v.as_str()));
            add_row("Description", meta.get("description").and_then(|v| v.as_str()));
            add_row("Keywords", meta.get("keywords").and_then(|v| v.as_str()));
            add_row("What's New", meta.get("whatsNew").and_then(|v| v.as_str()));
        } else {
            add_row("Locale", Some(default_locale));
            add_row("Warning", Some("No metadata found for default locale"));
        }

        format!("{}\n{}\n\n{}\n\n", "Default Locale".bold(), "—".repeat(18), table)
    }

    /// Show a completeness table for the default locale and derive an overall status
    /// Status: 上线 / 未上线 / 有错误
    pub fn format_completeness_table(data: &Value) -> String {
        let mut lines: Vec<String> = Vec::new();

        let default_locale = data
            .get("defaultLocale")
            .and_then(|v| v.as_str())
            .unwrap_or("en-US");

        let app_id = data.get("appId").and_then(|v| v.as_str()).unwrap_or("");
        let version = data.get("appVersion").and_then(|v| v.as_str()).unwrap_or("1.0.0");

        let meta = data
            .get("metadata")
            .and_then(|m| m.get(default_locale))
            .cloned()
            .unwrap_or(Value::Null);

        let obj = meta.as_object();
        let name_ok = obj.and_then(|o| o.get("name")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
        let subtitle_ok = obj.and_then(|o| o.get("subtitle")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
        let desc_len = obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).map(|s| s.len()).unwrap_or(0);
        let keywords_len = obj.and_then(|o| o.get("keywords")).and_then(|v| v.as_str()).map(|s| s.len()).unwrap_or(0);
        let whats_new_ok = obj.and_then(|o| o.get("whatsNew")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);

        // Validate constraints
        let mut errors: Vec<String> = Vec::new();
        if desc_len > 4000 { errors.push(format!("Description too long ({} > 4000)", desc_len)); }
        if keywords_len > 100 { errors.push(format!("Keywords exceed 100 chars ({} > 100)", keywords_len)); }

        // Screenshots: read manifest `<PWD>/<appId>/<version>/<locale>/screenshots.json`
        let mut screenshot_total: usize = 0;
        let mut screenshot_err = None;
        if !app_id.is_empty() {
            let mut p = PathBuf::new();
            if let Ok(cwd) = std::env::current_dir() {
                p.push(cwd);
            }
            p.push(app_id);
            p.push(version);
            p.push(default_locale);
            p.push("screenshots.json");
            match fs::read_to_string(&p) {
                Ok(s) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&s) {
                        if let Some(sets) = json.get("sets").and_then(|v| v.as_array()) {
                            for set in sets {
                                if let Some(items) = set.get("items").and_then(|v| v.as_array()) {
                                    screenshot_total += items.len();
                                }
                            }
                        }
                    } else {
                        screenshot_err = Some("Invalid screenshots.json".to_string());
                    }
                }
                Err(_) => {
                    screenshot_err = Some("screenshots.json not found".to_string());
                }
            }
        }

        // Compose table
        lines.push("🧪 完整性检查".to_string());
        lines.push("=".repeat(30));
        lines.push(String::new());

        let mut add_row = |item: &str, ok: bool, note: String| {
            let status = if ok { "✅ 完成" } else { "⚠️  缺失" };
            lines.push(format!("- {:<12} | {:<6} | {}", item, status, note));
        };

        add_row("App Name", name_ok, String::new());
        add_row("Subtitle", subtitle_ok, String::new());
        add_row("Description", desc_len > 0, format!("{} chars", desc_len));
        add_row("Keywords", keywords_len > 0, format!("{} / 100 chars", keywords_len));
        add_row("What's New", whats_new_ok, String::new());

        let screenshots_ok = screenshot_total > 0;
        let screenshots_note = if let Some(e) = screenshot_err { e } else { format!("{} images", screenshot_total) };
        add_row("Screenshots", screenshots_ok, screenshots_note);

        lines.push(String::new());

        // Determine overall status
        let overall_status = if !errors.is_empty() {
            "有错误"
        } else if name_ok && desc_len > 0 && keywords_len > 0 && screenshots_ok {
            "上线"
        } else {
            "未上线"
        };

        if !errors.is_empty() {
            lines.push("错误:".to_string());
            for e in errors { lines.push(format!("- {}", e)); }
            lines.push(String::new());
        }

        lines.push(format!("上线状态: {}", overall_status));
        lines.push(String::new());

        lines.join("\n")
    }

    /// Show a compact multi-locale status table
    /// Columns: Locale | Text | Screenshots | Status
    pub fn format_locales_status_table(data: &Value) -> String {
        let mut output = String::new();

        let app_id = data.get("appId").and_then(|v| v.as_str()).unwrap_or("");
        let version = data.get("appVersion").and_then(|v| v.as_str()).unwrap_or("1.0.0");
        let default_locale = data.get("defaultLocale").and_then(|v| v.as_str()).unwrap_or("en-US");

        // Collect locales
        let mut locales: Vec<String> = data
            .get("locales")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
            .unwrap_or_default();

        // Ensure metadata keys are also included
        if let Some(meta) = data.get("metadata").and_then(|v| v.as_object()) {
            for k in meta.keys() {
                if !locales.contains(k) { locales.push(k.clone()); }
            }
        }

        // Order: default locale first, then others sorted
        locales.sort();
        if let Some(pos) = locales.iter().position(|l| l == default_locale) {
            let def = locales.remove(pos);
            locales.insert(0, def);
        }

        // Table header
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(vec![
            Cell::new("Locale").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Text").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Screenshots").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Status").add_attribute(comfy_table::Attribute::Bold),
        ]);

        for locale in locales {
            // Text completeness
            let meta = data.get("metadata").and_then(|m| m.get(&locale)).cloned().unwrap_or(Value::Null);
            let obj = meta.as_object();
            let name_ok = obj.and_then(|o| o.get("name")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
            let subtitle_ok = obj.and_then(|o| o.get("subtitle")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);
            let desc_len = obj.and_then(|o| o.get("description")).and_then(|v| v.as_str()).map(|s| s.len()).unwrap_or(0);
            let keywords_len = obj.and_then(|o| o.get("keywords")).and_then(|v| v.as_str()).map(|s| s.len()).unwrap_or(0);
            let whats_new_ok = obj.and_then(|o| o.get("whatsNew")).and_then(|v| v.as_str()).map(|s| !s.trim().is_empty()).unwrap_or(false);

            let text_ok_count = (name_ok as u8)
                + (subtitle_ok as u8)
                + ((desc_len > 0) as u8)
                + ((keywords_len > 0 && keywords_len <= 100) as u8)
                + (whats_new_ok as u8);

            // Errors
            let mut has_error = false;
            if desc_len > 4000 { has_error = true; }
            if keywords_len > 100 { has_error = true; }

            // Screenshots count from manifest
            let mut screenshots = 0usize;
            if !app_id.is_empty() {
                let mut p = PathBuf::new();
                if let Ok(cwd) = std::env::current_dir() { p.push(cwd); }
                p.push(app_id);
                p.push(version);
                p.push(&locale);
                p.push("screenshots.json");
                if let Ok(s) = fs::read_to_string(&p) {
                    if let Ok(json) = serde_json::from_str::<Value>(&s) {
                        if let Some(sets) = json.get("sets").and_then(|v| v.as_array()) {
                            for set in sets {
                                if let Some(items) = set.get("items").and_then(|v| v.as_array()) {
                                    screenshots += items.len();
                                }
                            }
                        }
                    }
                }
            }

            // Overall status
            let status = if has_error { "有错误" } else if text_ok_count >= 4 && screenshots > 0 { "上线" } else { "未上线" };
            let status_colored = match status {
                "上线" => status.green().to_string(),
                "未上线" => status.yellow().to_string(),
                _ => status.red().to_string(),
            };

            table.add_row(vec![
                if locale == default_locale { locale.bold().to_string() } else { locale.to_string() },
                format!("{}/5", text_ok_count),
                format!("{}", screenshots),
                status_colored,
            ]);
        }

        format!("{}\n{}\n\n{}\n", "🌍 多语言状态".bold(), "—".repeat(18), table)
    }
    
    /// Format a single metadata field with appropriate icon and wrapping
    fn format_metadata_field(field: &str, value: &str) -> String {
        let (icon, label) = match field {
            "name" => ("📱", "App Name"),
            "description" => ("📝", "Description"),
            "keywords" => ("🔍", "Keywords"),
            "whatsNew" => ("📢", "What's New"),
            "subtitle" => ("📋", "Subtitle"),
            _ => ("📄", field),
        };
        
        let wrapped_value = Self::wrap_text(value, 60);
        format!("   {} {}: {}\n", icon, label, wrapped_value)
    }
    
    /// Wrap long text with proper indentation
    fn wrap_text(text: &str, width: usize) -> String {
        if text.len() <= width {
            return text.to_string();
        }
        
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        for word in words {
            if current_line.len() + word.len() + 1 <= width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                }
                current_line.push_str(word);
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        // Join lines with proper indentation for continuation
        if lines.len() == 1 {
            lines[0].clone()
        } else {
            let mut result = lines[0].clone();
            for line in &lines[1..] {
                result.push_str(&format!("\n      {}", line));
            }
            result
        }
    }
    
    /// Get emoji flag for locale
    fn get_locale_flag(locale: &str) -> &'static str {
        match locale {
            "en-US" => "🇺🇸",
            "zh-Hans" => "🇨🇳", 
            "zh-Hant" => "🇹🇼",
            "fr-FR" => "🇫🇷",
            "de-DE" => "🇩🇪",
            "ja-JP" => "🇯🇵",
            "ko-KR" => "🇰🇷",
            "es-ES" => "🇪🇸",
            "pt-BR" => "🇧🇷",
            "it-IT" => "🇮🇹",
            "ru-RU" => "🇷🇺",
            "ar-SA" => "🇸🇦",
            _ => "🌐",
        }
    }
    
    /// Format translation results in a clean table-like format
    pub fn format_translation_results(translations: &Value, source_locale: &str) -> String {
        let mut output = String::new();
        
        output.push_str("📊 Translation Results\n");
        output.push_str(&"═".repeat(50));
        output.push('\n');
        
        if let Some(translations_obj) = translations.as_object() {
            // Sort locales for consistent display
            let mut sorted_translations: BTreeMap<String, &Value> = BTreeMap::new();
            for (locale, data) in translations_obj {
                if locale != source_locale {
                    sorted_translations.insert(locale.clone(), data);
                }
            }
            
            for (locale, locale_data) in sorted_translations {
                output.push_str(&Self::format_translation_summary(&locale, locale_data));
                output.push('\n');
            }
        }
        
        output
    }
    
    /// Format a summary of translations for one locale
    fn format_translation_summary(locale: &str, data: &Value) -> String {
        let mut output = String::new();
        let locale_flag = Self::get_locale_flag(locale);
        
        output.push_str(&format!("\n{} {} Translation\n", locale_flag, locale));
        output.push_str(&"─".repeat(30));
        output.push('\n');
        
        if let Some(obj) = data.as_object() {
            if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                output.push_str(&format!("📱 {}\n", name));
            }
            
            if let Some(desc) = obj.get("description").and_then(|v| v.as_str()) {
                let preview = if desc.len() > 80 {
                    format!("{}...", &desc[..77])
                } else {
                    desc.to_string()
                };
                output.push_str(&format!("📝 {}\n", preview));
            }
            
            if let Some(keywords) = obj.get("keywords").and_then(|v| v.as_str()) {
                let preview = if keywords.len() > 50 {
                    format!("{}...", &keywords[..47])
                } else {
                    keywords.to_string()
                };
                output.push_str(&format!("🔍 {}\n", preview));
            }
        }
        
        output
    }
    
    /// Format cost information in a structured way
    pub fn format_cost_info(cost_data: &Value) -> String {
        let mut output = String::new();
        
        output.push_str("💰 Cost Information\n");
        output.push_str(&"═".repeat(30));
        output.push('\n');
        
        if let Some(cost) = cost_data.get("totalCost").and_then(|c| c.as_f64()) {
            output.push_str(&format!("💵 Total Cost: ${:.4}\n", cost));
        } else if let Some(cost) = cost_data.get("estimatedCost").and_then(|c| c.as_f64()) {
            output.push_str(&format!("💵 Estimated Cost: ${:.4}\n", cost));
        }
        
        if let Some(tokens) = cost_data.get("tokensUsed").and_then(|t| t.as_object()) {
            if let (Some(input), Some(output_tokens)) = (
                tokens.get("input").and_then(|i| i.as_u64()),
                tokens.get("output").and_then(|o| o.as_u64())
            ) {
                output.push_str(&format!("🔤 Input Tokens: {}\n", input));
                output.push_str(&format!("🔤 Output Tokens: {}\n", output_tokens));
                output.push_str(&format!("🔤 Total Tokens: {}\n", input + output_tokens));
            }
        } else if let Some(token_estimate) = cost_data.get("tokenEstimate").and_then(|t| t.as_u64()) {
            output.push_str(&format!("🔤 Token Estimate: {}\n", token_estimate));
        }
        
        output.push('\n');
        output
    }
}
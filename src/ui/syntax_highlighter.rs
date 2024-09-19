use crate::message::Message;
use crate::ui::themes::Theme;
use egui::{text::LayoutJob, FontId, TextFormat, Color32};
use regex::Regex;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use std::collections::HashMap;
use std::sync::Mutex;

const MAX_CACHE_SIZE: usize = 1000;

pub struct SyntaxHighlighter {
    ss: SyntaxSet,
    ts: ThemeSet,
    cache: Mutex<HashMap<String, (Vec<HighlightedBlock>, usize)>>,
    cache_counter: Mutex<usize>,
}

#[derive(Clone)]
pub enum HighlightedBlock {
    Text(LayoutJob),
    Code {
        language: String,
        job: LayoutJob,
    },
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            ss: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
            cache: Mutex::new(HashMap::new()),
            cache_counter: Mutex::new(0),
        }
    }

    pub fn highlight_message(&self, content: &str, is_user: bool, theme: &Theme, use_light_syntax: bool, is_streaming: bool) -> Vec<HighlightedBlock> {
        let cache_key = format!("{}-{}-{}-{}-{}", content, is_user, theme.name, use_light_syntax, is_streaming);
        let mut cache = self.cache.lock().unwrap();
        let mut counter = self.cache_counter.lock().unwrap();

        if let Some((blocks, _)) = cache.get(&cache_key) {
            return blocks.clone();
        }

        let blocks = self.generate_highlighted_blocks(content, is_user, theme, use_light_syntax, is_streaming);
        
        *counter += 1;
        cache.insert(cache_key, (blocks.clone(), *counter));

        if cache.len() > MAX_CACHE_SIZE {
            let oldest = cache.iter().min_by_key(|(_, (_, count))| count).map(|(k, _)| k.clone());
            if let Some(oldest_key) = oldest {
                cache.remove(&oldest_key);
            }
        }

        blocks
    }

    fn generate_highlighted_blocks(&self, content: &str, is_user: bool, theme: &Theme, use_light_syntax: bool, is_streaming: bool) -> Vec<HighlightedBlock> {
        let mut blocks = Vec::new();
        let re = Regex::new(r"```(\w+)?").unwrap();
        let mut last_end = 0;
        let mut in_code_block = false;
        let mut current_language = String::new();

        for cap in re.captures_iter(content) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();

            if !in_code_block {
                if start > last_end {
                    blocks.push(self.create_text_block(&content[last_end..start], is_user, theme));
                }
                in_code_block = true;
                current_language = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| "text".to_string());
                last_end = end;
            } else {
                blocks.push(self.create_code_block(&content[last_end..start], &current_language, theme, use_light_syntax));
                in_code_block = false;
                last_end = end;
            }
        }

        if last_end < content.len() {
            if in_code_block {
                let mut code_content = content[last_end..].to_string();
                if is_streaming {
                    code_content.push_str("\n```");
                }
                blocks.push(self.create_code_block(&code_content, &current_language, theme, use_light_syntax));
            } else {
                blocks.push(self.create_text_block(&content[last_end..], is_user, theme));
            }
        }

        blocks
    }

    fn create_text_block(&self, text: &str, is_user: bool, theme: &Theme) -> HighlightedBlock {
        let mut job = LayoutJob::default();
        job.append(
            text,
            0.0,
            TextFormat {
                font_id: FontId::new(18.0, egui::FontFamily::Proportional),
                color: if is_user { theme.user_text_color } else { theme.bot_text_color },
                ..Default::default()
            },
        );
        HighlightedBlock::Text(job)
    }

    fn create_code_block(&self, code: &str, language: &str, theme: &Theme, use_light_syntax: bool) -> HighlightedBlock {
        let mut job = LayoutJob::default();
        let syntax = self.ss.find_syntax_by_token(language).unwrap_or_else(|| self.ss.find_syntax_plain_text());
        let syntax_theme = if use_light_syntax {
            &self.ts.themes["base16-ocean.light"]
        } else {
            &self.ts.themes["base16-ocean.dark"]
        };
        let mut h = HighlightLines::new(syntax, syntax_theme);

        for line in LinesWithEndings::from(code) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ss).unwrap();
            for (style, text) in ranges {
                job.append(
                    text,
                    0.0,
                    TextFormat {
                        font_id: FontId::new(16.0, egui::FontFamily::Monospace),
                        color: Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b),
                        ..Default::default()
                    },
                );
            }
        }

        HighlightedBlock::Code {
            language: language.to_string(),
            job,
        }
    }

    pub fn clear_cache(&self) {
        self.cache.lock().unwrap().clear();
        *self.cache_counter.lock().unwrap() = 0;
    }
}
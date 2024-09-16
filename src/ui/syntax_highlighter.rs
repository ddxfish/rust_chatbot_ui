use crate::message::Message;
use crate::ui::themes::Theme;
use egui::{text::LayoutJob, FontId, TextFormat, Color32};
use regex::Regex;
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    ss: SyntaxSet,
    ts: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            ss: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight_message(&self, message: &Message, theme: &Theme) -> LayoutJob {
        let mut job = LayoutJob::default();
        let re = Regex::new(r"```(\w+)?\n([\s\S]*?)\n```").unwrap();
        let mut last_end = 0;

        for cap in re.captures_iter(message.content()) {
            let start = cap.get(0).unwrap().start();
            let end = cap.get(0).unwrap().end();

            // Add non-code text
            if start > last_end {
                job.append(
                    &message.content()[last_end..start],
                    0.0,
                    TextFormat {
                        font_id: FontId::new(16.0, egui::FontFamily::Proportional),
                        color: if message.is_user() { theme.user_text_color } else { theme.bot_text_color },
                        ..Default::default()
                    },
                );
            }

            // Highlight code
            let lang = cap.get(1).map(|m| m.as_str()).unwrap_or("text");
            let code = cap.get(2).unwrap().as_str();
            let syntax = self.ss.find_syntax_by_token(lang).unwrap_or_else(|| self.ss.find_syntax_plain_text());
            let mut h = HighlightLines::new(syntax, &self.ts.themes["base16-ocean.dark"]);

            for line in LinesWithEndings::from(code) {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &self.ss).unwrap();
                for (style, text) in ranges {
                    job.append(
                        text,
                        0.0,
                        TextFormat {
                            font_id: FontId::new(14.0, egui::FontFamily::Monospace),
                            color: Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b),
                            ..Default::default()
                        },
                    );
                }
            }

            last_end = end;
        }

        // Add remaining text
        if last_end < message.content().len() {
            job.append(
                &message.content()[last_end..],
                0.0,
                TextFormat {
                    font_id: FontId::new(16.0, egui::FontFamily::Proportional),
                    color: if message.is_user() { theme.user_text_color } else { theme.bot_text_color },
                    ..Default::default()
                },
            );
        }

        job
    }
}
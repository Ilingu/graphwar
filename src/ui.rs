use std::time::{Duration, Instant};

use egui::{Color32, FontId, RichText};

#[derive(Clone, Copy)]
pub enum UITypes {
    Neutral,
    Info,
    Success,
    Warning,
    Error,
}

pub fn rich_text(text: &str, text_type: UITypes) -> RichText {
    RichText::new(text)
        .color(match text_type {
            UITypes::Neutral => Color32::WHITE,
            UITypes::Info => Color32::LIGHT_BLUE,
            UITypes::Success => Color32::LIGHT_GREEN,
            UITypes::Warning => Color32::LIGHT_YELLOW,
            UITypes::Error => Color32::LIGHT_RED,
        })
        .font(FontId::monospace(16.0))
}

pub struct Message {
    content: String,
    duration: Duration,
    msg_type: UITypes,
    now: Instant,
}

impl Message {
    pub fn new(content: String, duration: Duration, msg_type: UITypes) -> Self {
        Self {
            content,
            duration,
            msg_type,
            now: Instant::now(),
        }
    }
    pub fn default() -> Self {
        Message {
            content: String::new(),
            duration: Duration::from_secs(0),
            msg_type: UITypes::Neutral,
            now: Instant::now(),
        }
    }

    /*
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    */

    pub fn render(&self) -> RichText {
        rich_text(&self.content, self.msg_type)
    }

    pub fn is_expired(&self) -> bool {
        self.now.elapsed() >= self.duration
    }
}

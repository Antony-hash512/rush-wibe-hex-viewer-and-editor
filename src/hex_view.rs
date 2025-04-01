use gtk::prelude::*;
use gtk::{Box, ScrolledWindow, TextView};
use std::fs::File;
use std::io::{self, Read};

#[derive(Clone)]
pub struct HexView {
    container: Box,
    text_view: TextView,
}

impl HexView {
    pub fn new() -> Self {
        let container = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .build();

        let text_view = TextView::builder()
            .editable(false)
            .monospace(true)
            .build();

        let scrolled_window = ScrolledWindow::builder()
            .child(&text_view)
            .build();

        container.append(&scrolled_window);

        Self {
            container,
            text_view,
        }
    }

    pub fn get_container(&self) -> &Box {
        &self.container
    }

    pub fn load_file(&self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut hex_display = String::new();
        let mut offset = 0;

        for chunk in buffer.chunks(16) {
            // Добавляем смещение
            hex_display.push_str(&format!("{:08x}  ", offset));
            
            // Добавляем hex-значения
            for (i, &byte) in chunk.iter().enumerate() {
                hex_display.push_str(&format!("{:02x} ", byte));
                if i == 7 {
                    hex_display.push_str(" ");
                }
            }

            // Дополняем строку пробелами, если не хватает байтов
            if chunk.len() < 16 {
                let padding = (16 - chunk.len()) * 3;
                hex_display.push_str(&" ".repeat(padding));
            }

            // Добавляем ASCII-представление
            hex_display.push_str(" |");
            for &byte in chunk {
                if byte.is_ascii_graphic() || byte == b' ' {
                    hex_display.push(byte as char);
                } else {
                    hex_display.push('.');
                }
            }
            hex_display.push_str("|\n");

            offset += chunk.len();
        }

        let buffer = self.text_view.buffer();
        buffer.set_text(&hex_display);

        Ok(())
    }
} 
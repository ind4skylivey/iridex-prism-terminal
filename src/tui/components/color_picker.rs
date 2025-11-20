use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render_picker<'a>(label: &str, hex: &str) -> Paragraph<'a> {
    Paragraph::new(Span::styled(
        format!("{label}: {hex}"),
        Style::default().fg(parse_color(hex)),
    ))
    .block(Block::default().borders(Borders::ALL).title("Color"))
}

fn parse_color(hex: &str) -> Color {
    if let Ok(rgb) = fast_color_hex::HexColor::parse_rgb(hex) {
        Color::Rgb(rgb.r, rgb.g, rgb.b)
    } else {
        Color::Reset
    }
}

mod fast_color_hex {
    pub struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    pub struct HexColor;

    impl HexColor {
        pub fn parse_rgb(input: &str) -> Result<Rgb, ()> {
            let clean = input.trim_start_matches('#');
            if clean.len() != 6 {
                return Err(());
            }
            let r = u8::from_str_radix(&clean[0..2], 16).map_err(|_| ())?;
            let g = u8::from_str_radix(&clean[2..4], 16).map_err(|_| ())?;
            let b = u8::from_str_radix(&clean[4..6], 16).map_err(|_| ())?;
            Ok(Rgb { r, g, b })
        }
    }
}

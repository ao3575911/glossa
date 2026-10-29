//! Brand colours from `brand/tokens.json` (graphite / iron-blue / vellum / rule / ink-muted).

use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub accent: Color,
    pub muted: Color,
    pub rule: Color,
    pub highlight: Color,
    pub match_node: Color,
    pub no_color: bool,
}

impl Theme {
    pub fn dark(no_color: bool) -> Self {
        Self {
            bg: Color::Rgb(28, 31, 36),
            fg: Color::Rgb(232, 230, 225),
            accent: Color::Rgb(47, 74, 110),
            muted: Color::Rgb(92, 99, 112),
            rule: Color::Rgb(154, 161, 171),
            highlight: Color::Rgb(47, 74, 110),
            match_node: Color::Rgb(232, 230, 225),
            no_color,
        }
    }

    pub fn title(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.accent)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn footer(&self) -> Style {
        Style::default().fg(self.muted)
    }

    pub fn row_selected(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().bg(self.highlight).fg(self.fg)
        }
    }

    pub fn match_style(&self) -> Style {
        if self.no_color {
            Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        } else {
            Style::default()
                .fg(self.match_node)
                .bg(self.accent)
                .add_modifier(Modifier::BOLD)
        }
    }
}

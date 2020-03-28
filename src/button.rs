use graphics::{Graphics, character::CharacterCache, Context, types::Color};
use piston::generic_event::GenericEvent;

pub struct Button {
    text: String,
    hovered: bool
}

impl Button {
    pub fn new(text: String) -> Self {
        Self {
            text,
            hovered: false
        }
    }
}

pub struct ButtonViewSettings
{
    pub position: [f64; 2],
    pub size: [f64; 2],
    pub background_color: Color,
    pub hovered_background_color: Color,
    pub border_color: Color,
    pub border_radius: f64,
    pub text_color: Color,
    pub font_size: u32
}

impl ButtonViewSettings {
    pub fn new(position: [f64; 2], size: [f64; 2]) -> Self {
        Self {
            position,
            size,
            background_color: [0.8, 0.8, 1.0, 1.0],
            hovered_background_color: [0.9, 0.9, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            border_radius: 2.0,
            text_color: [0.0, 0.0, 1.0, 1.0],
            font_size: 15
        }
    }
}

pub struct ButtonView {
    settings: ButtonViewSettings
}

impl ButtonView {
    pub fn new(settings: ButtonViewSettings) -> Self {
        Self {
            settings
        }
    }

    pub fn draw<G, C>(&mut self, button: &Button, c: &Context, g: &mut G, glyphs: &mut C)
        where G: Graphics, C: CharacterCache<Texture = G::Texture> {
        use graphics::*;

        let ref settings = self.settings;

        // Draw button background.
        let button_rect = [
            settings.position[0], settings.position[1],
            settings.size[0], settings.size[1]
        ];
        Rectangle::new(if button.hovered {
                settings.hovered_background_color
            } else {
                settings.background_color
            })
            .draw(button_rect, &c.draw_state, c.transform, g);

        // Draw button text.
        let width = glyphs.width(settings.font_size, &button.text)
            .map_err(|_| "Failed to get glyphs width!")
            .unwrap();
        let transform = c.transform.trans(settings.position[0] + (settings.size[0] - width) / 2.0,
            settings.position[1] + (settings.size[1] + settings.font_size as f64) / 2.0);
        Text::new_color(settings.text_color, settings.font_size)
            .round()
            .draw(&button.text, glyphs, &c.draw_state, transform, g)
            .map_err(|_| "Failed to render text!")
            .unwrap();

        // Draw button border.
        Rectangle::new_border(settings.border_color, settings.border_radius)
            .draw(button_rect, &c.draw_state, c.transform, g);
    }
}

pub struct ButtonController {
    button: Button,
    button_view: ButtonView,
    cursor_pos: [f64; 2]
}

impl ButtonController {
    pub fn new(button: Button, button_view: ButtonView) -> Self {
        Self {
            button,
            button_view,
            cursor_pos: [0.0; 2]
        }
    }

    pub fn draw<G, C>(&mut self, c: &Context, g: &mut G, glyphs: &mut C)
        where G: Graphics, C: CharacterCache<Texture = G::Texture> {

        self.button_view.draw(&self.button, c, g, glyphs);
    }

    // Handles events for button. Returns true if button was clicked.
    pub fn handle_event<E>(&mut self, e: &E) -> bool where E: GenericEvent {
        use piston::input::*;

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;

            // Check if mouse points at button.
            let (x, y) = (self.cursor_pos[0], self.cursor_pos[1]);
            let position = self.button_view.settings.position;
            let size = self.button_view.settings.size;
            if x >= position[0] && x <= position[0] + size[0] &&
                y >= position[1] && y <= position[1] + size[1] {

                self.button.hovered = true;
            } else {
                self.button.hovered = false;
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Check if mouse points at button.
            let (x, y) = (self.cursor_pos[0], self.cursor_pos[1]);
            let position = self.button_view.settings.position;
            let size = self.button_view.settings.size;
            if x >= position[0] && x <= position[0] + size[0] &&
                y >= position[1] && y <= position[1] + size[1] {

                return true;
            }
        }

        false
    }
}

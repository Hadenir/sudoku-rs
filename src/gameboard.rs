use graphics::{Graphics, character::CharacterCache, Context, types::Color};
use piston::generic_event::GenericEvent;
use std::collections::BTreeSet;

// Size of gameboard.
const SIZE: usize = 9;

// Stores information about single cell.
#[derive(Copy, Clone)]
struct Cell {
    digit: u8, // 0 means no digit is written.
    notes: [bool; 9] // Describes which digit is pencil-marked in the cell.
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            digit: 0,
            notes: [false; 9]
        }
    }
}

// Stores information about game board.
pub struct Gameboard {
    // Contents of cells.
    // 0 means empty cell.
    cells: [[Cell; SIZE]; SIZE],
    selected_cell: Option<[usize; 2]>
}

impl Gameboard {
    pub fn new() -> Self {
        Self {
            cells: [[Cell::default(); SIZE]; SIZE],
            selected_cell: None
        }
    }

    // Returns digit written in cell.
    pub fn get_digit(&self, ind: [usize; 2]) -> Option<u8> {
        let digit = self.cells[ind[1]][ind[0]].digit;

        if digit == 0 {
            None
        } else {
            Some(digit)
        }
    }

    // Returns notes put in cell.
    pub fn get_notes(&self, ind: [usize; 2]) -> [bool; 9] {
        return self.cells[ind[1]][ind[0]].notes;
    }

    // Writes single digit in cell.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]].digit = val;
    }

    // Notes digit in cell. If digit is already noted, removes it.
    pub fn note(&mut self, ind: [usize; 2], val: u8) {
        let ref mut cell = self.cells[ind[1]][ind[0]];
        let i = (val - 1) as usize;
        cell.notes[i] = !cell.notes[i];
    }
}

// Stores settings for game board view.
pub struct GameboardViewSettigs {
    // Position from top-left corner.
    pub position: [f64; 2],
    // Size along horizontal and vertical edge.
    pub size: f64,
    // Color of background.
    pub background_color: Color,
    // Color of board border.
    pub border_color: Color,
    // Color of edge around board.
    pub board_edge_color: Color,
    // Color of edge around 3x3 section.
    pub section_edge_color: Color,
    // Color of edge around single cell.
    pub cell_edge_color: Color,
    // Backgrond color of selected cell.
    pub selected_cell_background_color: Color,
    // Radius of edge around board.
    pub board_edge_radius: f64,
    // Radius of edge around 3x3 section.
    pub section_edge_radius: f64,
    // Radius of edge around single cell.
    pub cell_edge_radius: f64,
    // Color of font.
    pub text_color: Color,
    // Size of font.
    pub font_size: u32,
    // Color of font for notes.
    pub note_color: Color,
    // Size of font for notes.
    pub note_font_size: u32
}

impl Default for GameboardViewSettigs {
    fn default() -> Self {
        Self {
            position: [56.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            text_color: [0.0, 0.0, 1.0, 1.0],
            font_size: 34,
            note_color: [0.37, 0.37, 0.63, 1.0],
            note_font_size: 10
        }
    }
}

pub struct GameboardView {
    settings: GameboardViewSettigs
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettigs) -> Self {
        Self {
            settings
        }
    }

    pub fn draw<G, C>(&mut self, gameboard: &Gameboard, c: &Context, g: &mut G, glyphs: &mut C)
        where G: Graphics, C: CharacterCache<Texture = G::Texture> {
        use graphics::*;

        let ref settings = self.settings;
        let cell_size = settings.size / 9.0;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size
        ];

        // Draw board background.
        Rectangle::new(settings.background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);

        // Draw selected cell background.
        if let Some(ind) = gameboard.selected_cell {
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0], settings.position[1] + pos[1],
                cell_size, cell_size
            ];

            Rectangle::new(settings.selected_cell_background_color)
                .draw(cell_rect, &c.draw_state, c.transform, g);
        }

        // Draw digits.
        for j in 0..9 {
            for i in 0..9 {
                let pos = [
                    settings.position[0] + i as f64 * cell_size,
                    settings.position[1] + j as f64 * cell_size
                ];

                if let Some(digit) = gameboard.get_digit([i, j]) {
                    let text_image = Image::new_color(settings.text_color);
                    if let Ok(character) = glyphs.character(settings.font_size,
                        GameboardView::get_char(digit)) {

                        let ch_x = pos[0] + (cell_size - character.atlas_size[0]) / 2.0;
                        let ch_y = pos[1] + (cell_size - character.atlas_size[1]) / 2.0;

                        let text_image = text_image.src_rect([
                            character.atlas_offset[0],
                            character.atlas_offset[1],
                            character.atlas_size[0],
                            character.atlas_size[1]
                        ]);

                        let transform = c.transform.trans(ch_x, ch_y);
                        text_image.draw(character.texture, &c.draw_state, transform, g);
                    }
                } else {
                    let notes = gameboard.get_notes([i, j]);
                    let text_image = Image::new_color(settings.note_color);
                    for n in 0..9 {
                        if notes[n] {
                            if let Ok(character) = glyphs.character(settings.note_font_size,
                                GameboardView::get_char((n + 1) as u8)) {

                                // let ch_x = pos[0] + cell_size / 6.0 - character.atlas_size[0] / 2.0 + cell_size / 3.0 * (n % 3) as f64;
                                // let ch_y = pos[1] + cell_size / 6.0 - character.atlas_size[1] / 2.0 + cell_size / 3.0 * (n / 3) as f64;

                                let ch_x = pos[0] + cell_size / 3.0 * (0.5 + (n % 3) as f64)
                                    - character.atlas_size[0] / 2.0;
                                let ch_y = pos[1] + cell_size / 3.0 * (0.5 + (n / 3) as f64)
                                    - character.atlas_size[1] / 2.0;

                                let text_image = text_image.src_rect([
                                    character.atlas_offset[0],
                                    character.atlas_offset[1],
                                    character.atlas_size[0],
                                    character.atlas_size[1]
                                ]);

                                let transform = c.transform.trans(ch_x, ch_y);
                                text_image.draw(character.texture, &c.draw_state, transform, g);
                            }
                        }
                    }
                }
            }
        }

        // Draw grid.
        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            let hline = [settings.position[0], y, x2, y];

            if i % 3 == 0 {
                section_edge.draw(vline, &c.draw_state, c.transform, g);
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            } else {
                cell_edge.draw(vline, &c.draw_state, c.transform, g);
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
            .draw(board_rect, &c.draw_state, c.transform, g);
    }

    fn get_char(val: u8) -> char {
        match val {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => '0'    // Should never happen.
        }
    }
}

pub struct GameboardController {
    gameboard: Gameboard,
    gameboard_view: GameboardView,
    cursor_pos: [f64; 2],
    shift_pressed: bool
}

impl GameboardController {
    pub fn new(gameboard: Gameboard, gameboard_view: GameboardView) -> Self {
        Self {
            gameboard,
            gameboard_view,
            cursor_pos: [0.0; 2],
            shift_pressed: false
        }
    }

    pub fn check(&self) -> bool {
        let ref gameboard = self.gameboard;

        let mut occurrences = BTreeSet::new();

        for row in 0..9 {
            occurrences.clear();
            for column in 0..9 {
                let digit = gameboard.cells[row][column].digit;
                if digit == 0 || occurrences.contains(&digit) {
                    return false;
                } else {
                    occurrences.insert(digit);
                }
            }
        }

        for column in 0..9 {
            occurrences.clear();
            for row in 0..9 {
                let digit = gameboard.cells[row][column].digit;
                if occurrences.contains(&digit) {
                    return false;
                } else {
                    occurrences.insert(digit);
                }
            }
        }

        for section in 0..9 {
            occurrences.clear();
            for i in 0..9 {
                let column = (section % 3) * 3 + i % 3;
                let row = (section / 3) * 3 + i / 3;
                let digit = gameboard.cells[row][column].digit;
                if occurrences.contains(&digit) {
                    return false;
                } else {
                    occurrences.insert(digit);
                }
            }
        }

        true
    }

    pub fn draw<G, C>(&mut self, c: &Context, g: &mut G, glyphs: &mut C)
        where G: Graphics, C: CharacterCache<Texture = G::Texture> {

        self.gameboard_view.draw(&self.gameboard, c, g, glyphs);
    }

    pub fn handle_event<E>(&mut self, e: &E) where E: GenericEvent {
        use piston::input::*;

        let pos = self.gameboard_view.settings.position;
        let size = self.gameboard_view.settings.size;

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to top-left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];

            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;

                self.gameboard.selected_cell = Some([cell_x, cell_y]);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::LShift {
                self.shift_pressed = true;
            }

            if let Some(ind) = self.gameboard.selected_cell {
                if self.shift_pressed {
                    match key {
                        Key::D1 => self.gameboard.note(ind, 1),
                        Key::D2 => self.gameboard.note(ind, 2),
                        Key::D3 => self.gameboard.note(ind, 3),
                        Key::D4 => self.gameboard.note(ind, 4),
                        Key::D5 => self.gameboard.note(ind, 5),
                        Key::D6 => self.gameboard.note(ind, 6),
                        Key::D7 => self.gameboard.note(ind, 7),
                        Key::D8 => self.gameboard.note(ind, 8),
                        Key::D9 => self.gameboard.note(ind, 9),
                        Key::Escape => self.gameboard.set(ind, 0),
                        _ => ()
                    }
                } else {
                    match key {
                        Key::D1 => self.gameboard.set(ind, 1),
                        Key::D2 => self.gameboard.set(ind, 2),
                        Key::D3 => self.gameboard.set(ind, 3),
                        Key::D4 => self.gameboard.set(ind, 4),
                        Key::D5 => self.gameboard.set(ind, 5),
                        Key::D6 => self.gameboard.set(ind, 6),
                        Key::D7 => self.gameboard.set(ind, 7),
                        Key::D8 => self.gameboard.set(ind, 8),
                        Key::D9 => self.gameboard.set(ind, 9),
                        Key::Escape => self.gameboard.set(ind, 0),
                        _ => ()
                    }
                }
            }
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            if key == Key::LShift {
                self.shift_pressed = false;
            }
        }
    }
}

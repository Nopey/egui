use crate::{font::Font, layout, style, types::GuiInput, Frame, Painter, RawInput};

/// Encapsulates input, layout and painting for ease of use.
#[derive(Clone)]
pub struct Emgui {
    pub last_input: RawInput,
    pub data: layout::Data,
    pub style: style::Style,
    pub painter: Painter,
}

impl Emgui {
    pub fn new(font: Font) -> Emgui {
        Emgui {
            last_input: Default::default(),
            data: layout::Data::new(font.clone()),
            style: Default::default(),
            painter: Painter::new(font),
        }
    }

    pub fn texture(&self) -> (u16, u16, &[u8]) {
        self.painter.texture()
    }

    pub fn new_frame(&mut self, new_input: RawInput) {
        let gui_input = GuiInput::from_last_and_new(&self.last_input, &new_input);
        self.last_input = new_input;
        self.data.new_frame(gui_input);
    }

    pub fn whole_screen_region(&mut self) -> layout::Region {
        layout::Region {
            data: &mut self.data,
            id: Default::default(),
            dir: layout::Direction::Vertical,
            cursor: Default::default(),
            size: Default::default(),
        }
    }

    pub fn set_options(&mut self, options: layout::LayoutOptions) {
        self.data.options = options;
    }

    pub fn paint(&mut self) -> Frame {
        let gui_commands = self.data.gui_commands();
        let paint_commands = style::into_paint_commands(gui_commands, &self.style);
        self.painter.paint(&paint_commands)
    }
}

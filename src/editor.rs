use game::*;
use renderer::*;
use resource_manager::*;
use ui_renderer::*;

pub struct Editor {}

impl Editor {
    pub fn new() -> Editor {
        Editor {}
    }

    pub fn render_editor(
        &mut self,
        ui: &mut UIContext,
        game_state: &mut GameState,
        context: &mut RenderContext,
        res: &mut ResourceContext,
    ) {
        if game_state.menu_open {
            return;
        }

        let scrn_width = ui.win_width;
        let scrn_height = ui.win_height;

        let win_width = scrn_width / 8.0;
        let win_height = scrn_height;

        let element_padding = 20.0;

        let win_pos_x = scrn_width - win_width;
        let win_pos_y = 0.0;

        let win_top_pos = (win_pos_y + win_height) - element_padding;

        let button_width = win_width - element_padding;
        let button_height = 15.0;

        // println!(
        //     "Screen Size {}, {} \n Editor Win Size {}, {} \n",
        //     scrn_width, scrn_height, win_width, win_height
        // );

        ui.set_font_size(20);

        ui.render_quad(
            game_state.default_ui,
            win_pos_x,
            win_pos_y,
            win_width,
            win_height,
        );

        let mut element_idx = 0;

        ui.render_text(
            "Editor",
            win_pos_x,
            win_top_pos
                - button_height / 2.0
                - ((button_height + element_padding) * element_idx as f32),
        );
    }
}

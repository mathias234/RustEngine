use game::*;
use ui_renderer::*;

pub fn render_editor(ui: &mut UIContext, game_state: &mut GameState) {
    if game_state.menu_open {
        return;
    }

    let width = ui.win_width;
    let height = ui.win_height;

    ui.set_font_size(40);

    ui.render_button(
        game_state.default_ui,
        "Spawn Obj",
        width / 2.0,
        height / 2.0,
        50.0,
        15.0,
    );
}

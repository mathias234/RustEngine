use assets;
use game::*;
use gameobject::*;
use glium::glutin;
use input::*;
use material::*;
use model::Model;
use quaternion::*;
use renderer::*;
use resource_manager::*;
use shader;
use texture;
use ui_renderer::*;
use vector::*;

pub struct Editor {
    gizmo_x: Option<Resource>,
    gizmo_y: Option<Resource>,
    gizmo_z: Option<Resource>,
    selected_obj: Option<Resource>,
}

impl Editor {
    pub fn new(
        display: &glium::Display,
        rc: &mut RenderContext,
        res: &mut ResourceContext,
    ) -> Editor {
        let gizmo_model_x = res.alloc_model(Model::load(
            &display,
            &assets::get_asset("./res/gizmo/gizmo_x_axis.obj"),
            &assets::get_asset("./res/gizmo/gizmo_x_axis.mtl"),
        ));

        let gizmo_model_y = res.alloc_model(Model::load(
            &display,
            &assets::get_asset("./res/gizmo/gizmo_y_axis.obj"),
            &assets::get_asset("./res/gizmo/gizmo_y_axis.mtl"),
        ));

        let gizmo_model_z = res.alloc_model(Model::load(
            &display,
            &assets::get_asset("./res/gizmo/gizmo_z_axis.obj"),
            &assets::get_asset("./res/gizmo/gizmo_z_axis.mtl"),
        ));

        let basic_shader = res.alloc_shader(shader::load(
            &display,
            &assets::get_asset("./res/basic.vs"),
            &assets::get_asset("./res/basic.fs"),
        ));

        let white = res.alloc_tex(texture::load(
            &display,
            &assets::get_asset("./res/white.jpg"),
        ));

        let ground_material =
            Material::new(basic_shader, white, white, [1.0, 1.0, 1.0], [5.0, 5.0]);

        let mut gizmo_x = GameObject::new(
            res,
            "Gizmo x".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
            gizmo_model_x,
            ground_material,
        );

        let mut gizmo_y = GameObject::new(
            res,
            "Gizmo y".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
            gizmo_model_y,
            ground_material,
        );

        let mut gizmo_z = GameObject::new(
            res,
            "Gizmo z".to_string(),
            Vector3::new(0.0, 0.0, 0.0),
            Quaternion::new(0.0, 0.0, 0.0, 1.0),
            gizmo_model_z,
            ground_material,
        );

        gizmo_x.overlay = true;
        gizmo_x.material.color = [1.0, 0.0, 0.0];
        gizmo_y.overlay = true;
        gizmo_y.material.color = [0.0, 1.0, 0.0];
        gizmo_z.overlay = true;
        gizmo_z.material.color = [0.0, 0.0, 1.0];

        let gizmo_x = res.alloc_gameobject(gizmo_x);
        let gizmo_y = res.alloc_gameobject(gizmo_y);
        let gizmo_z = res.alloc_gameobject(gizmo_z);

        rc.gameobjects.push(gizmo_x);
        rc.gameobjects.push(gizmo_y);
        rc.gameobjects.push(gizmo_z);

        Editor {
            gizmo_x: Some(gizmo_x),
            gizmo_y: Some(gizmo_y),
            gizmo_z: Some(gizmo_z),
            selected_obj: None,
        }
    }

    pub fn is_gizmo(&self, id: Option<Resource>) -> bool {
        let unwrapped = id.unwrap();

        (unwrapped == self.gizmo_x.unwrap())
            || (unwrapped == self.gizmo_y.unwrap())
            || (unwrapped == self.gizmo_z.unwrap())
    }

    pub fn update(
        &mut self,
        game_state: &mut GameState,
        rc: &mut RenderContext,
        res: &mut ResourceContext,
        input: &mut Input,
    ) {
        if input.get_mouse_down(glutin::MouseButton::Left) && rc.picked_object.is_some() {
            if self.is_gizmo(rc.picked_object) {

            } else {
                self.selected_obj = rc.picked_object;
            }
        }

        if self.selected_obj.is_some() {
            let selected_obj_pos = res.get_gameobject_ref(self.selected_obj.unwrap()).position;

            {
                let gizmo = res.get_gameobject_ref_mut(self.gizmo_x.unwrap());
                gizmo.position = selected_obj_pos;
            }
            {
                let gizmo = res.get_gameobject_ref_mut(self.gizmo_y.unwrap());
                gizmo.position = selected_obj_pos;
            }
            {
                let gizmo = res.get_gameobject_ref_mut(self.gizmo_z.unwrap());
                gizmo.position = selected_obj_pos;
            }
        }
    }

    pub fn render_editor(&mut self, ui: &mut UIContext, game_state: &mut GameState) {
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

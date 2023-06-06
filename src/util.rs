
use eframe::egui;

#[inline]
pub fn get_wnd_center_pos(ctx: &egui::Context, width: f32, height: f32) -> (f32, f32, egui::Pos2) {
    let main_wnd_size = ctx.input(|state| state.screen_rect.center());
    (
        width,
        height,
        // egui::Pos2::new(0., 0.),
        egui::Pos2::new(main_wnd_size.x - width / 2., main_wnd_size.y - height / 2.),
    )
}


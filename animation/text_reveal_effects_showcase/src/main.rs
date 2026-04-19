/// Demonstration of both typewriter and centered reveal text effects
/// Shows the difference between fixed-anchor and shifting text animations
use glam::{Vec3, Vec4};
use murali::App;
use murali::positions::*;
use murali::colors::*;
use murali::engine::scene::Scene;
use murali::engine::timeline::Timeline;
use murali::frontend::animation::Ease;
use murali::frontend::collection::text::label::Label;

fn main() -> anyhow::Result<()> {
    let mut scene = Scene::new();
    let mut timeline = Timeline::new();

    // ===== TYPEWRITER MODE (Fixed Position) =====
    let mut typewriter_label =
        Label::new("Typewriter Effect", 0.5).with_color(GREEN_D);
    typewriter_label.typewriter_mode = true; // Explicitly set typewriter mode
    let typewriter_id = scene.add_tattva(typewriter_label, 4.0 * LEFT + 2.0 * UP);

    // ===== REVEAL MODE (Shifting/Growing from Center) =====
    let reveal_label =
        Label::new("Reveal Effect", 0.5).with_color(RED_B);
    let reveal_id = scene.add_tattva(reveal_label, 2.0 * RIGHT + 2.0 * UP);

    scene.camera_mut().position = CAMERA_DEFAULT_POS;

    // ===== TYPEWRITER ANIMATIONS =====
    // Typewrite label
    timeline
        .animate(typewriter_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .typewrite_text()
        .spawn();

    // ===== REVEAL ANIMATIONS =====
    // Reveal label (grows from center)
    timeline
        .animate(reveal_id)
        .at(0.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .reveal_text()
        .spawn();

    // ===== UNWRITE ANIMATIONS =====
    // Reverse typewriter label
    timeline
        .animate(typewriter_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .untypewrite_text()
        .spawn();

    // ===== UNREVEAL ANIMATIONS =====
    // Hide revealed label (shrinks to center)
    timeline
        .animate(reveal_id)
        .at(4.0)
        .for_duration(2.0)
        .ease(Ease::Linear)
        .hide_text()
        .spawn();

    scene.set_timeline("main", timeline);

    App::new()?.with_scene(scene).run_app()
}

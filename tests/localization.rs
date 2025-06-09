use gero::localization::Localizer;
use gero::ui::{UiManager, UiTab};
use gero::frontend::Renderer;

#[test]
fn localizer_loads_file_and_returns_value() {
    let loc = Localizer::new("en").unwrap();
    assert_eq!(loc.get("ui.tab.abilities"), "Abilities");
}

#[test]
fn ui_render_uses_localized_strings() {
    let mut ui = UiManager::new(50, 50, vec![], vec![]);
    ui.spawn_floating_text(-3, (1, 1));
    let mut renderer = Renderer::new_headless(50, 50);
    let loc = Localizer::new("en").unwrap();
    ui.render(&mut renderer, &loc);
    assert!(renderer.draw_log.iter().any(|c| c.sprite_id == "float:damage:3"));
    assert_eq!(UiTab::Abilities.label(&loc), "Abilities");
}

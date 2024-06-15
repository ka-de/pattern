// SetFont
use bevy::{
    asset::AssetServer,
    ecs::system::EntityCommand,
    prelude::{ Entity, World },
    render::color::Color,
    text::Text,
    ui::{ PositionType, Style, Val },
};

pub struct SetFont(pub String, pub f32, pub Color);

impl EntityCommand for SetFont {
    fn apply(self, entity: Entity, world: &mut World) {
        let asset_server = world.resource::<AssetServer>();
        let font = asset_server.load(&self.0);

        if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
            for text_section in &mut text.sections {
                text_section.style.font = font.clone();
                text_section.style.font_size = self.1;
                text_section.style.color = self.2;
            }
        }
    }
}

// SetPosition
pub struct SetPosition(pub f32, pub f32);

impl EntityCommand for SetPosition {
    fn apply(self, entity: Entity, world: &mut World) {
        // Commands work with direct access to the world.
        // We can set the position by modifying the style directly:
        if let Some(mut style) = world.entity_mut(entity).get_mut::<Style>() {
            style.position_type = PositionType::Absolute;
            style.left = Val::Px(self.0);
            style.top = Val::Px(self.1);
            style.right = Val::Auto;
            style.bottom = Val::Auto;
        }

        // Because you have access to the world, you could access resources or perform queries here.
    }
}

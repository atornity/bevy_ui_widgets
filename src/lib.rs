#![allow(clippy::type_complexity)]

use bevy_app::{PluginGroup, PluginGroupBuilder};

pub mod components;
pub mod theming;
pub mod utils;
pub mod widgets;

/// Plugin group used to add every plugins offered by bevy_ui_widget at once.
/// Use this if you don't want to mix and match which plugins you need.
pub struct AllWidgetsPlugins;

impl PluginGroup for AllWidgetsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(components::grab::GrabComponentsPlugin)
            .add(components::toggle::ToggleComponentsPlugin)
            .add(theming::ThemingPlugin)
            .add(widgets::frame::FramePlugin)
            .add(widgets::tooltip::TooltipPlugin)
            .add(widgets::slider::SliderPlugin)
    }
}

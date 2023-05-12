use bevy::prelude::*;
use bevy_ui_widgets::{components::toggle::Toggle, theming::*, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .add_system(toggle_theme)
        .run();
}

fn setup(mut commands: Commands, mut theme: ResMut<ThemeManager>) {
    commands.spawn(Camera2dBundle::default());

    // static theme variables
    theme
        .set_property("root", FlexDirectionProperty(FlexDirection::ColumnReverse))
        .set_property("root", JustifyContentProperty(JustifyContent::Center))
        .set_property("root", AlignItemsProperty(AlignItems::Center))
        .set_property(
            "root",
            SizeProperty(Size::new(Val::Percent(100.0), Val::Percent(100.0))),
        )
        .set_property("root", ColorProperty(Color::WHITE))
        .set_property("button", JustifyContentProperty(JustifyContent::Center))
        .set_property("button", AlignItemsProperty(AlignItems::Center))
        .set_property("button", PaddingProperty(UiRect::all(Val::Px(10.0))))
        .set_property("button", ColorProperty(Color::rgb(0.15, 0.15, 0.15)))
        .set_property(
            "text",
            TextFontPathProperty("fonts/FiraSans-Bold.ttf".into()),
        )
        .set_property("text", TextFontSizeProperty(20.0))
        .set_property("text", TextColorProperty(Color::RED));

    let root = commands
        .spawn(NodeBundle::default())
        .insert(ThemeKey("root".into()))
        .id();

    let text = commands
        .spawn(TextBundle::from_section(
            "Click here to toggle theme",
            default(),
        ))
        .insert(ThemeKey("text".into()))
        .id();

    let button = commands
        .spawn(ButtonBundle::default())
        .insert(ThemeKey("button".into()))
        .insert(Toggle::default())
        .add_child(text)
        .id();

    commands.entity(root).add_child(button);
}

/// Changes some theme properties when the button is toggled.
/// This basically shows how a light/dark mode could be achieved.
///
/// TODO: try to implement this via swapping pre-filled themes and using insert.
fn toggle_theme(mut query: Query<&Toggle, Changed<Toggle>>, mut theme: ResMut<ThemeManager>) {
    for toggle in query.iter_mut() {
        match toggle {
            Toggle::Off => {
                theme
                    .set_property("root", ColorProperty(Color::BLACK))
                    .set_property("text", TextColorProperty(Color::GREEN));
            }
            Toggle::On => {
                theme
                    .set_property("root", ColorProperty(Color::GRAY))
                    .set_property("text", TextColorProperty(Color::RED));
            }
        }
    }
}

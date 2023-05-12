use bevy::prelude::*;
use bevy_ui_widgets::{components::toggle::Toggle, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AllWidgetsPlugins)
        .add_startup_system(setup)
        .add_system(toggle_color)
        .run();
}

const TOGGLE_OFF_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const TOGGLE_ON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(TOGGLE_OFF_COLOR),
            ..default()
        })
        .insert(Toggle::default());
}

#[allow(clippy::type_complexity)]
fn toggle_color(
    mut query: Query<
        (&Toggle, &Interaction, &mut BackgroundColor),
        Or<(Changed<Toggle>, Changed<Interaction>)>,
    >,
) {
    for (toggle, interaction, mut color) in query.iter_mut() {
        let lighten_value = match interaction {
            Interaction::Clicked => -0.04,
            Interaction::Hovered => 0.04,
            Interaction::None => 0.0,
        };

        color.0 = match toggle {
            Toggle::Off => lighten(TOGGLE_OFF_COLOR, lighten_value),
            Toggle::On => lighten(TOGGLE_ON_COLOR, lighten_value),
        }
    }
}

fn lighten(color: Color, value: f32) -> Color {
    if let Color::Hsla {
        hue,
        saturation,
        lightness,
        alpha,
    } = color.as_hsla()
    {
        Color::Hsla {
            hue,
            saturation,
            lightness: lightness + value,
            alpha,
        }
    } else {
        default()
    }
}

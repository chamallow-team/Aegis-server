use bevy::prelude::*;

#[derive(Component)]
pub struct IdleColor(BackgroundColor);

pub(crate) fn setup_ui(
    mut commands: Commands
)
{
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(200.),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,

            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            left: Val::Px(0.),

            padding: UiRect::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    padding: UiRect::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),

                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                ..default()
            },
            IdleColor(BackgroundColor(Color::rgb(0.2, 0.2, 0.2)))
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Map",
                    TextStyle {
                        font_size: 28.,
                        color: Color::WHITE,
                        ..default()
                    }
                ),
                ..default()
            });
        });

        parent.spawn((
            ButtonBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    padding: UiRect::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),

                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                ..default()
            },
            IdleColor(BackgroundColor(Color::rgb(0.2, 0.2, 0.2)))
        )).with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Tech. Tree",
                    TextStyle {
                        font_size: 28.,
                        color: Color::WHITE,
                        ..default()
                    }
                ),
                ..default()
            });
        });
    });
}

pub(crate) fn button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &IdleColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut button_color, IdleColor(idle_color)) in interaction_query.iter_mut() {
        *button_color = match interaction {
            Interaction::Hovered => Color::rgb(0.5, 0.5, 0.5).into(),
            _ => *idle_color,
        };
    }
}

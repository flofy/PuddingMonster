use bevy::prelude::*;

#[derive(Component)]
pub struct Star {
    pub collected: bool,
}

#[derive(Component)]
pub struct StarCollector;

pub fn setup_stars(
    commands: &mut Commands,
    star_positions: &[Vec2],
) {
    for position in star_positions {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(position.x, position.y, 1.0),
                ..default()
            },
            Star { collected: false },
        ));
    }
}

pub fn check_star_collection(
    mut commands: Commands,
    pudding_q: Query<&Transform, With<Pudding>>,
    mut star_q: Query<(Entity, &Transform, &mut Star)>,
    mut star_events: EventWriter<StarCollectedEvent>,
) {
    for (star_entity, star_transform, mut star) in star_q.iter_mut() {
        if !star.collected {
            for pudding_transform in pudding_q.iter() {
                let pudding_pos = pudding_transform.translation.truncate();
                let star_pos = star_transform.translation.truncate();
                let distance = (pudding_pos - star_pos).length();

                if distance < 35.0 { // Pudding size (40) + star size (30) / 2
                    star.collected = true;
                    star_events.send(StarCollectedEvent { star: star_entity });
                    commands.entity(star_entity).despawn();
                    break;
                }
            }
        }
    }
}

#[derive(Event)]
pub struct StarCollectedEvent {
    pub star: Entity,
}

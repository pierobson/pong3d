use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const BALL_SCALE: f32 = 3.0;

#[derive(Debug, Default)]
enum Team {
    #[default]
    Left,
    Right,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Team::Left => write!(f, "Left"),
            Team::Right => write!(f, "Right"),
        }
    }
}

#[derive(Default)]
enum InputYDirection {
    #[default]
    None,
    Up,
    Down,
}

#[derive(Default)]
enum InputXDirection {
    #[default]
    None,
    Left,
    Right,
}

#[derive(Component, Default)]
struct Paddle {
    team: Team,
    velocity: Vec3,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Goal;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<RapierConfiguration>,
) {
    let paddle_mesh = meshes.add(shape::Cube::default().into());
    let ball_mesh = meshes.add(shape::UVSphere::default().into());

    let paddle_scale = Vec3::new(4.0, 12.0, 5.0);
    let ball_scale = Vec3::new(BALL_SCALE, BALL_SCALE, BALL_SCALE);

    config.gravity = Vec3::default();

    let paddle1_transform = Vec3::new(-50.0, 0.0, 0.0);

    // Setup paddle
    commands
        .spawn(PbrBundle {
            mesh: paddle_mesh.clone(),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(paddle1_transform.clone())
                .with_scale(paddle_scale.clone()),
            ..Default::default()
        })
        .insert(Paddle {
            team: Team::Left,
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));

    let paddle2_transform = Vec3::new(50.0, 0.0, 0.0);

    commands
        .spawn(PbrBundle {
            mesh: paddle_mesh.clone(),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(paddle2_transform.clone())
                .with_scale(paddle_scale.clone()),
            ..Default::default()
        })
        .insert(Paddle {
            team: Team::Right,
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(KinematicCharacterController::default())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));

    // Setup ball
    commands
        .spawn(PbrBundle {
            mesh: ball_mesh,
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(ball_scale),
            ..Default::default()
        })
        .insert(Ball)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(1.0))
        .insert(Restitution {
            coefficient: 1.05,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Velocity::linear(Vec3::new(-100.0, 0.0, 0.0)));

    // Add walls at +-40y for the ball to bounce off of.

    commands
        .spawn(Wall)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -40.0, 0.0)))
        .insert(RigidBody::Fixed)
        .insert(Collider::halfspace(Vect::new(0.0, 1.0, 0.0)).unwrap());

    commands
        .spawn(Wall)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 40.0, 0.0)))
        .insert(RigidBody::Fixed)
        .insert(Collider::halfspace(Vect::new(0.0, -1.0, 0.0)).unwrap());

    
    // TODO: Add walls to box in either end of the game and the midpoint as part of a 
    //       collision group to box in the Paddles but not affect the ball.


    // TODO: Add Sensors behind the paddles (+-60?) for scoring.

    commands
        .spawn(Goal)
        .insert(TransformBundle::from(Transform::from_xyz(-60.0, 0.0, 0.0)))
        .insert(Collider::halfspace(Vect::new(1.0, 1.0, 0.0)).unwrap())
        .insert(Sensor);

    commands
        .spawn(Goal)
        .insert(TransformBundle::from(Transform::from_xyz(60.0, 0.0, 0.0)))
        .insert(Collider::halfspace(Vect::new(-1.0, 1.0, 0.0)).unwrap())
        .insert(Sensor);

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 16000.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Setup camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn player_controller(mut paddle_query: Query<&mut Paddle>, input: Res<Input<KeyCode>>) {
    for mut paddle in paddle_query.iter_mut() {
        let mut y_dir = InputYDirection::default();
        let mut x_dir = InputXDirection::default();

        match paddle.team {
            Team::Left => {
                if input.pressed(KeyCode::W) {
                    y_dir = InputYDirection::Up;
                } else if input.pressed(KeyCode::S) {
                    y_dir = InputYDirection::Down;
                }

                if input.pressed(KeyCode::A) {
                    x_dir = InputXDirection::Left;
                } else if input.pressed(KeyCode::D) {
                    x_dir = InputXDirection::Right;
                }
            }
            Team::Right => {
                if input.pressed(KeyCode::P) {
                    y_dir = InputYDirection::Up;
                } else if input.pressed(KeyCode::Semicolon) {
                    y_dir = InputYDirection::Down;
                }

                if input.pressed(KeyCode::L) {
                    x_dir = InputXDirection::Left;
                } else if input.pressed(KeyCode::Apostrophe) {
                    x_dir = InputXDirection::Right;
                }
            }
        }

        match y_dir {
            InputYDirection::None => {
                paddle.velocity.y = 0.0;
            }
            InputYDirection::Up => {
                paddle.velocity.y = 1.0;
            }
            InputYDirection::Down => {
                paddle.velocity.y = -1.0;
            }
        }

        match x_dir {
            InputXDirection::None => {
                paddle.velocity.x = 0.0;
            }
            InputXDirection::Left => {
                paddle.velocity.x = -1.0;
            }
            InputXDirection::Right => {
                paddle.velocity.x = 1.0;
            }
        }
    }
}

fn update_character_controller(
    mut character_query: Query<(&mut KinematicCharacterController, &Paddle), With<Paddle>>,
) {
    for (mut controller, paddle) in character_query.iter_mut() {
        controller.translation = match controller.translation {
            Some(t) => Some(t + paddle.velocity),
            None => Some(paddle.velocity),
        }
    }
}


#[cfg(debug_assertions)]
fn debug_system(paddle_loc: Query<(&Transform, &Paddle)>) {
    for (transform, paddle) in paddle_loc.iter() {
        println!(
            "Paddle {} at ({},{})",
            paddle.team, transform.translation.x, transform.translation.y
        );
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Update, debug_system);
    }

    app.add_systems(Startup, setup)
        .add_systems(Update, (player_controller, update_character_controller))
        .run();
}

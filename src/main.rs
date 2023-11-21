use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, parry::shape::HalfSpace};

const BALL_SCALE: f32 = 3.0;

#[derive(Debug)]
enum Team {
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
enum InputDirection {
    #[default]
    None,
    Up,
    Down,
}

#[derive(Component)]
struct Paddle {
    team: Team,
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;


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

    // Setup paddle
    commands
        .spawn(PbrBundle {
            mesh: paddle_mesh.clone(),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(-50.0, 0.0, 0.0))
                .with_scale(paddle_scale.clone()),
            ..Default::default()
        })
        .insert(Paddle { team: Team::Left })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Velocity::default());

    commands
        .spawn(PbrBundle {
            mesh: paddle_mesh,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 0.0))
                .with_scale(paddle_scale.clone()),
            ..Default::default()
        })
        .insert(Paddle { team: Team::Right })
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Velocity::default());

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
        .insert(Velocity::linear(Vec3::new(-40.0, 0.0, 0.0)));

    // TODO: Add walls at +-40y for the ball to bounce off of.

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


    // commands
    //     .spawn(Wall)
    //     .insert(RigidBody::Fixed)
    //     .insert(Collider::cuboid(60.0, 20.0, 5.0))
    //     .insert(Transform::from_translation(Vec3::new(0.0, 41.0, 0.0)));

    // TODO: Add Sensors behind the paddles (+-60?) for scoring + resetting.

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

fn player_controller(
    mut paddle_query: Query<(&Paddle, &mut Velocity)>,
    input: Res<Input<KeyCode>>,
) {
    for (paddle, mut velocity) in paddle_query.iter_mut() {
        let mut dir = InputDirection::default();

        match paddle.team {
            Team::Left => {
                if input.pressed(KeyCode::W) {
                    dir = InputDirection::Up;
                } else if input.pressed(KeyCode::S) {
                    dir = InputDirection::Down;
                }
            }
            Team::Right => {
                if input.pressed(KeyCode::P) {
                    dir = InputDirection::Up;
                } else if input.pressed(KeyCode::L) {
                    dir = InputDirection::Down;
                }
            }
        }

        match dir {
            InputDirection::None => {
                *velocity = Velocity::linear(Vec3::default());
            }
            InputDirection::Up => {
                *velocity = Velocity::linear(Vec3::new(0.0, 50.0, 0.0));
            }
            InputDirection::Down => {
                *velocity = Velocity::linear(Vec3::new(0.0, -50.0, 0.0));
            }
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

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugins(RapierDebugRenderPlugin::default());
        app.add_systems(Update, debug_system);
    }

    app.add_systems(Startup, setup)
        .add_systems(Update, player_controller)
        .run();
}

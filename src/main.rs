#![allow(clippy::type_complexity)]
use bevy::{prelude::*, app::AppExit};

mod bomber;
use bomber::Bomber;
use rand::random;

mod ai;

const FIELD_SIZE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "I am a window!".into(),
            width: 500.,
            height: 300.,
            vsync: true,
            ..Default::default()
        })
        .add_state_to_stage(CoreStage::Update, AppState::Running)
        .add_state_to_stage(CoreStage::PostUpdate, AppState::Running)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppState::Running)
            .with_system(
                move_ai_bomber_system
                // .chain(print_ai_positions_system)
                .chain(bomber_hit_detection_system)
            )
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(AppState::Running)
            // .with_system(print_bombers_system)
            .with_system(game_over_system)
            .with_system(bomber::render_system)
            .with_system(window_title_system)
        )
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Running,
    Finished,
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(bomber::white())
        .insert(ai::Controller::new());

    commands
        .spawn_bundle(bomber::black())
        .insert(ai::Controller::new());
    
    commands
        .spawn_bundle(bomber::red())
        .insert(ai::Controller::new());
    
    commands
        .spawn_bundle(bomber::blue())
        .insert(ai::Controller::new());

    for _ in 0..100 {
        commands
            .spawn_bundle(bomber::random())
            .insert(ai::Controller::new());
    }
    
    let field_dim = bomber::SPRITE_SIZE * FIELD_SIZE * 2. + bomber::SPRITE_SIZE;
    // let field_pos = 
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(field_dim, field_dim)),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Component, Debug, Default)]
pub struct Position {
    x: f32,
    y: f32,
}

fn print_bombers_system(
    bombers: Query<&Bomber>,
) {
    for bomber in bombers.iter() {
        println!("{:?}", bomber);
    }
}

fn print_ai_positions_system(
    query: Query<(&Bomber, &Position, &ai::Controller)>,
) {
    for (bomber, position, controller) in query.iter() {
        if !controller.timer.just_finished() {
            continue;
        }

        println!("{:?} @ ({}, {})", bomber.color, position.x, position.y);
    }
}

#[derive(Component)]
struct Alive;

fn move_ai_bomber_system(
    mut query: Query<
        (&mut Position, &mut ai::Controller),
        (With<Bomber>, With<Alive>),
    >,
    time: Res<Time>,
) {
    for (mut position, mut controller) in query.iter_mut() {
        if !controller.timer.tick(time.delta()).just_finished() {
            continue;
        }

        let direction = random::<ai::Direction>();

        match direction {
            ai::Direction::Up => position.y -= 1.,
            ai::Direction::Down => position.y += 1.,
            ai::Direction::Left => position.x -= 1.,
            ai::Direction::Right => position.x += 1.,
        }
    }
}

fn bomber_hit_detection_system(
    query: Query<
        (Entity, &Position),
        (With<Alive>, With<Bomber>),
    >,
    mut commands: Commands,
) {
    for (entity, position) in query.iter() {
        if position.x.abs() > FIELD_SIZE || position.y.abs() > FIELD_SIZE {
            commands.entity(entity).remove::<Alive>();
        }
    }
}

fn game_over_system(
    died_bombers: RemovedComponents<Alive>,
    dead_bombers: Query<&Bomber, Without<Alive>>,
    alive_bombers: Query<&Bomber, With<Alive>>,
    mut state: ResMut<State<AppState>>,
) {
    for bomber in died_bombers.iter() {
        if let Ok(bomber) = dead_bombers.get(bomber) {
            println!("{:?} bomber died!", bomber.color);
        }
    }

    let num_alive_bombers =  alive_bombers.iter().count();

    if num_alive_bombers >= 2 {
        return;
    }

    if num_alive_bombers == 0 {
        println!("You ALL suck at this");
    } else if let Ok(bomber) = alive_bombers.get_single() {
        println!("{:?} bomber wins!", bomber.color);
    }

    state.set(AppState::Finished).ok();
}

fn window_title_system(
    query: Query<(), (With<Bomber>, With<Alive>)>,
    mut windows: ResMut<Windows>,
) {
    let num_alive = query.iter().count();
    let window = windows.get_primary_mut().unwrap();

    window.set_title(format!("Num alive: {}", num_alive));
}
#[derive(Resource)]
struct SquareSprites {
    red: Sprite,
    green: Sprite,
    blue: Sprite,
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct SquareTimer {
    timer: Timer,
    step: u32,
}

impl Plugin for SquareCycle {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_square_timer);
        app.add_systems(Startup, generate_square_sprites);
        app.add_systems(Update, square_spawner);
    }
}
fn generate_square_sprites(mut commands: Commands) {
    let red: Sprite = Sprite {
        color: Color::srgb(1.0, 0.0, 0.0),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    };
    let green: Sprite = Sprite {
        color: Color::srgb(0.0, 1.0, 0.0),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    };
    let blue: Sprite = Sprite {
        color: Color::srgb(0.0, 0.0, 1.0),
        custom_size: Some(Vec2::new(50.0, 50.0)),
        ..default()
    };
    commands.insert_resource(SquareSprites { red, green, blue });
}

fn setup_square_timer(mut commands: Commands) {
    commands.spawn(SquareTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        step: 0,
    });
}

fn square_spawner(
    mut commands: Commands,
    time: Res<Time>,
    sprites: Res<SquareSprites>,
    mut timer_query: Query<&mut SquareTimer>,
    existing_squares: Query<Entity, With<Square>>,
) {
    let mut square_timer = timer_query.single_mut().unwrap();

    if square_timer.timer.tick(time.delta()).just_finished() {
        let step = square_timer.step % 4;
        square_timer.step += 1;

        let (sprite, position) = match step {
            0 => {
                for entity in existing_squares.iter() {
                    commands.entity(entity).despawn();
                }
                return;
            }
            1 => (sprites.red.clone(), -200.0),
            2 => (sprites.green.clone(), 0.0),
            3 => (sprites.blue.clone(), 200.0),
            _ => return,
        };
        commands.spawn((
            sprite,
            Transform {
                translation: Vec3::new(position, -200.0, 0.0),
                ..default()
            },
            Square,
        ));
    }
}

# Cheatsheet

table:

- Spawn Query Logic
- Transform
- Draw2D
- Text2D
- Input
- Gizmos
- Mouse Position
- Load Assets

## Spawn Query Logic

```rust
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn setup(mut commands: Commands) {
    commands.spawn(Car);
    commands.spawn((Transform::default(), Car)); // Car have Transform
}

fn update(
    // Query Transform that have Car
    query: Query<(&Transform, &Car)>,
    // Query Transform that have Car, without Car reference
    query: Query<&Transform, With<Car>>,
    // if there is conflict 2 query and mut / not, exclude: Without
    query: Query<&mut Transform, (With<Person>, Without<Car>)>,
) {
    for i in query { }
}
```

## Transform

```rust
fn handle(mut transform: Transform, time: Res<Time>) {
    let speed = 10.;

    // Move to Local Forward
    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = 10. * speed * time.delta_seconds();
    let translation_delta = movement_direction * movement_distance;

    transform.translation += translation_delta;


    // Rotate using degree
    transform.rotate_z(10. * speed * time.delta_seconds());


    // Rotate to vector direction
    let player_translation = player_transform.translation.xy();

    let to_player = (player_translation - enemy_transform.translation.xy()).normalize();

    let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));

    enemy_transform.rotation = rotate_to_player;
}
```

## Draw2D

```rust
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 1.0, 0.0, 0.3),
                ..default()
            },
            texture: asset_server.load("branding/icon.png"),
            ..default()
        },
        // tiling
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.5, // The image will tile every 128px
        },
    ));
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        material: materials.add(Color::WHITE),
        transform: Transform::default(),
        ..default()
    });
}
```

## Text2D

```rust
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 24.,
        color: Color::WHITE,
        ..default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "translation",
            text_style,
        )
            .with_justify(JustifyText::Center),
        ..default()
    })
    commands.spawn(TextBundle::from_section(
        "Some Text",
        text_style,
    ));

    // Text have Transform
}
```

## Input

```rust
fn update(
    keyboard: Res<ButtonInput<KeyCode>>,
    mb: Res<ButtonInput<MouseButton>>,

) {
    if keyboard.pressed(KeyCode::ArrowRight) { // Every frame }
    if keyboard.just_pressed(KeyCode::ArrowRight) { // One frame }
    if mb.just_pressed(MouseButton::Left) { }
}
```

## Gizmos

```rust
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyRoundGizmos {}

fn setup() {
    App::new()
        .init_gizmo_group::<MyRoundGizmos>()
        .run()
}

fn draw_example_collection(
    mut gizmos: Gizmos,
    mut my_gizmos: Gizmos<MyRoundGizmos>,
    time: Res<Time>,
) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    // The circles have 32 line-segments by default.
    my_gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    my_gizmos.ellipse_2d(
        Vec2::ZERO,
        time.elapsed_seconds() % TAU,
        Vec2::new(100., 200.),
        Color::YELLOW_GREEN,
    );
    // You may want to increase this for larger circles.
    my_gizmos
        .circle_2d(Vec2::ZERO, 300., Color::NAVY)
        .segments(64);

    // Arcs default amount of segments is linearly interpolated between
    // 1 and 32, using the arc length as scalar.
    my_gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);

    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(sin / -10. + PI / 2.) * 50.,
        Color::YELLOW,
    );
}
```

## Mouse Position

```rust
fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Some(cursor_position) = wins.single().cursor_position() else {
        return; // mouse outside window
    };

    let (camera, camera_transform) = camera_query.single();
    let Some(cursor) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    cursor.x;
    cursor.y;
}
```

## Load Assets

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        SpriteBundle {
            // inside assets directory in project root
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::default(),
            ..default()
        },
    );
}
```


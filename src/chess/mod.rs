//! 8| 56 57 58 59 60 61 62 63
//! 7| 48 49 50 51 52 53 54 55
//! 6| 40 41 42 43 44 45 46 47
//! 5| 32 33 34 35 36 37 38 39
//! 4| 24 25 26 27 28 29 30 31
//! 3| 16 17 18 19 20 21 22 23
//! 2| 8  9  10 11 12 13 14 15
//! 1| 0  1  2  3  4  5  6  7
//! __ a  b  c  d  e  f  g  h

use bevy::prelude::*;

pub fn plugins(app: &mut App) {
    app.add_plugins(DefaultPlugins);
    app.add_plugins(debug::plugins);
    app.add_systems(Startup, (setup,background::draw_tiles));
    app.add_systems(Update, (input,util::exit_on_q));
}

#[derive(Bundle)]
struct PieceBundle {
    sprite: SpriteBundle,
    ty: PieceType,
    index: Index,
    side: Side,
}

#[derive(Debug, Clone, Component, Deref, DerefMut, PartialEq)]
struct Index(u8);

#[derive(Debug, Component)]
enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}

#[derive(Debug, Component, PartialEq)]
enum Side {
    Light, Dark
}

#[derive(Debug, Component, PartialEq)]
enum SelectionState {
    None, Select(Entity)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SelectionState::None);

    let fenn = util::STARTING_FENN.chars();
    let mut i = 0;

    for f in fenn {
        if f == '/' {
            i = i - (i % 8) + 8;
            continue;
        }

        let ty = PieceType::from_fenn(f).expect("invalid starting fenn");
        let side = Side::from_fenn(f);
        let texture = asset_server.load(util::get_asset_name(&ty, &side));

        commands.spawn(PieceBundle {
            side, ty,
            index: Index(i),
            sprite: SpriteBundle {
                texture,
                transform: {
                    let Vec2 { x, y } = coordinate::vec2_from_index(i);
                    Transform::from_xyz(x,y,1.)
                },
                ..default()
            }
        });

        i += 1;
    }
}

fn input(
    mb: Res<ButtonInput<MouseButton>>,
    cam: Query<(&Camera, &GlobalTransform)>,
    win: Query<&Window>,
    mut commands: Commands,

    mut pieces: Query<(&mut Index, &mut Transform, Entity), With<Transform>>,
    mut state: Query<&mut SelectionState>,
) {
    if !mb.just_pressed(MouseButton::Left) { return }

    let Some(cursor) = util::get_mouse_pos(cam, win) else { return };
    if coordinate::is_outside_board(&cursor) { return }
    let clicked_index = coordinate::index_from_vec2(cursor);

    let mut state = state.single_mut();
    let target_piece = pieces.iter().find(|e|**e.0 == clicked_index);

    match *state {
        SelectionState::None => {
            let Some(target_piece) = target_piece else { return };
            *state = SelectionState::Select(target_piece.2);
        }
        SelectionState::Select(selected_piece) => {
            if let Some(target_piece) = target_piece {
                commands.entity(target_piece.2).despawn();
            }

            let Ok((mut piece_index, mut piece_pos, _)) = pieces.get_mut(selected_piece) else { return };

            *piece_index = Index(clicked_index);
            *piece_pos = {
                let Vec2 { x, y } = coordinate::vec2_from_index(clicked_index);
                Transform::from_xyz(x, y, 1.)
            };

            *state = SelectionState::None;
        },
    }
}

mod coordinate {
    use bevy::prelude::Vec2;

    pub const GRID_SCALE: f32 = 50.;

    const BOARD_HALF_SCALE: f32 = GRID_SCALE * 8. / 2.;
    const BOARD_OFFSET_VEC: Vec2 = Vec2::splat(-BOARD_HALF_SCALE);
    const ORIGIN_OFFSET: Vec2 = Vec2::splat(GRID_SCALE / 2.);

    pub fn vec2_from_index(i: u8) -> Vec2 {
        let i = i as f32;
        Vec2::new(i % 8., (i - (i % 8.)) / 8.) * GRID_SCALE + BOARD_OFFSET_VEC
    }

    pub fn index_from_vec2(vec2: Vec2) -> u8 {
        let clamped = (vec2 - BOARD_OFFSET_VEC + ORIGIN_OFFSET) / GRID_SCALE;
        let Vec2 { x, y } = clamped.trunc().abs();
        x as u8 + y as u8 * 8
    }

    pub fn is_outside_board(vec2: &Vec2) -> bool {
        vec2.x < -BOARD_HALF_SCALE
        || vec2.x > BOARD_HALF_SCALE
        || vec2.y < -BOARD_HALF_SCALE
        || vec2.y > BOARD_HALF_SCALE
    }
}

mod background {
    use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
    use super::*;
    pub const TILE_COUNT: u8 = 64;

    pub fn draw_tiles(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let rect_mesh = Mesh2dHandle(meshes.add(Rectangle::from_size(Vec2::splat(coordinate::GRID_SCALE))));
        let light_mat = materials.add(Color::WHITE);
        let dark_mat = materials.add(Color::MIDNIGHT_BLUE);

        for i in 0..TILE_COUNT {
            commands.spawn(MaterialMesh2dBundle {
                mesh: rect_mesh.clone(),
                transform: {
                    let Vec2 { x, y } = coordinate::vec2_from_index(i);
                    Transform::from_xyz(x, y, 0.)
                },
                material: match Side::from_index(i) {
                    Side::Light => light_mat.clone(),
                    Side::Dark => dark_mat.clone(),
                },
                ..default()
            });
        }
    }
}

mod util {
    use super::*;
    pub const STARTING_FENN: &str = "rnbqkbnrpppppppp////PPPPPPPPRNBQKBNR";

    pub fn get_asset_name<'r>(ty: &PieceType, side: &Side) -> String {
        let s = if side == &Side::Light { 'l' } else { 'd' };
        let w = match ty {
            PieceType::King => 'k',
            PieceType::Queen => 'q',
            PieceType::Bishop => 'b',
            PieceType::Knight => 'n',
            PieceType::Rook => 'r',
            PieceType::Pawn => 'p',
        };

        format!("{w}{s}t.png")
    }

    pub fn exit_on_q(key: Res<ButtonInput<KeyCode>>, mut ev: EventWriter<bevy::app::AppExit>) {
        if key.just_pressed(KeyCode::KeyQ) { ev.send_default(); }
    }

    pub fn get_mouse_pos(
        cam: Query<(&Camera, &GlobalTransform)>,
        win: Query<&Window>,
    ) -> Option<Vec2> {
        let mouse_viewport = win.single().cursor_position()?;
        let (cam,cam_pos) = cam.single();
        cam.viewport_to_world_2d(cam_pos, mouse_viewport)
    }

    impl PieceType {
        pub fn from_fenn(f: char) -> Option<Self> {
            let p = match f {
                'k'|'K' => Self::King,
                'q'|'Q' => Self::Queen,
                'b'|'B' => Self::Bishop,
                'n'|'N' => Self::Knight,
                'r'|'R' => Self::Rook,
                'p'|'P' => Self::Pawn,
                _ => return None
            };

            Some(p)
        }
    }

    impl Side {
        pub fn from_index(value: u8) -> Self {
            let a = value % 8;
            let b = (value - a) / 8;
            if (a + b) % 2 == 0 { Self::Light } else { Self::Dark }
        }
        pub fn from_fenn(f: char) -> Self {
            if f.is_uppercase() { Self::Dark } else { Self::Light }
        }
    }
}

mod debug {
    use super::*;

    pub fn plugins(app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update);
    }

    pub fn setup(mut commands: Commands) {
        commands.spawn(TextBundle {
            text: Text::from_section("Oof", default()),
            ..default()
        });
    }

    pub fn update(
        cam: Query<(&Camera, &GlobalTransform)>,
        win: Query<&Window>,
        state: Query<&SelectionState>,
        mut gizmos: Gizmos,
        mut text: Query<&mut Text>,
    ) {
        let Some(cursor) = util::get_mouse_pos(cam, win) else { return };
        let is_out = !coordinate::is_outside_board(&cursor);
        let state = state.single();
        let mut text = text.single_mut();

        let idx = coordinate::index_from_vec2(cursor);
        let clamped = coordinate::vec2_from_index(idx);

        gizmos.line_2d(Vec2::ZERO, cursor, Color::GREEN);
        gizmos.line_2d(Vec2::ZERO, clamped, Color::RED);

        text.sections[0].value = format!("State {state:?}\nIndex: {idx}\nCursor: {cursor:?}\nValid: {is_out}");
    }
}

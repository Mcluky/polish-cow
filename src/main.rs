use gemini_engine::{
    elements::{
        view::{ColChar, Wrapping},
        Vec2D, View,
    },
    elements3d::{
        view3d::LightType, DisplayMode, Face, Light, Mesh3D, Transform3D, Vec3D, ViewElement3D,
        Viewport,
    },
    gameloop,
};
use rodio::{source::Source, Decoder, OutputStream};
use std::io::Cursor;
use stl_io::IndexedMesh;

const FPS: f32 = 30.0;

fn main() {
    // get environment variables
    let args: Vec<String> = std::env::args().collect();

    // Embed the 3d cow file into the binary
    let stl_bytes = include_bytes!("../resources/lowpolycow.stl");
    // Create a Cursor to read from the byte slice
    let mut cursor = Cursor::new(stl_bytes);
    // Read the STL from the Cursor
    let stl = stl_io::read_stl(&mut cursor).unwrap();

    // embed the polish cow song into the binary
    let song_bytes = include_bytes!("../resources/polish-cow-song.mp3");
    // create a cursor to read the song from
    let song_cursor = Cursor::new(song_bytes);
    // check if the user has set no-sound as an argument
    let no_sound = args.len() > 1 && args[1] == "no-sound";
    // initialize the audio stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    if !no_sound {
        // create a decoder to decode the song
        let source = Decoder::new_looped(song_cursor).unwrap();
        // play the song
        stream_handle.play_raw(source.convert_samples()).unwrap();
    }

    let mesh = stl_to_3d_mesh(&stl);

    let mut view = View::new(55, 15, ColChar::EMPTY);
    let screen_origin = Vec2D::new(0, 0);
    let fov = 90.0;
    let mut transform_matrix = Transform3D::new_trs(
        Vec3D::new(15.0, -14.0, 110.0),
        Vec3D::new(0.0, 1.5, -1.5),
        Vec3D::new(0.3, 0.3, 0.3),
    );
    // let display_mode = DisplayMode::Wireframe { backface_culling: true };
    let light = Light {
        intensity: 1.2,
        light_type: LightType::Point {
            position: Vec3D::new(15.0, -10.0, 110.0),
        },
    };
    let display_mode = DisplayMode::Illuminated {
        lights: vec![light],
    };
    let meshes: Vec<&dyn ViewElement3D> = vec![&mesh];

    loop {
        view.clear();

        // Increase the rotation of the mesh around the y-axis by 0.1 radians
        transform_matrix.rotation.x += 0.1;
        let view_port = Viewport::new(transform_matrix, fov, screen_origin);
        let pixels = view_port.render(meshes.to_owned(), display_mode.to_owned());
        view.blit(&pixels, Wrapping::Ignore);

        let _ = view.display_render();

        let _ = gameloop::sleep_fps(FPS, None);
    }
}

fn stl_to_3d_mesh(stl_file: &IndexedMesh) -> Mesh3D {
    let vertices: Vec<Vec3D> = stl_file
        .vertices
        .iter()
        .map(|vertex| Vec3D::new(vertex[0].into(), vertex[1].into(), vertex[2].into()))
        .collect();

    let faces: Vec<Face> = stl_file
        .faces
        .iter()
        .map(|face| Face::new(face.vertices.to_vec(), ColChar::SOLID))
        .collect();

    Mesh3D::new(Transform3D::default(), vertices, faces)
}

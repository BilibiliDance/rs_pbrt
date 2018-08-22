extern crate getopts;
extern crate num_cpus;
extern crate pbrt;
// pest
extern crate pest;
#[macro_use]
extern crate pest_derive;

// parser
use pest::Parser;

// getopts
use getopts::Options;
// pbrt
use pbrt::accelerators::bvh::{BVHAccel, SplitMethod};
use pbrt::cameras::perspective::PerspectiveCamera;
use pbrt::core::camera::Camera;
use pbrt::core::film::Film;
use pbrt::core::filter::Filter;
use pbrt::core::geometry::{Bounds2f, Bounds2i, Normal3f, Point2f, Point2i, Point3f, Vector3f};
use pbrt::core::integrator::SamplerIntegrator;
use pbrt::core::light::Light;
use pbrt::core::material::Material;
use pbrt::core::medium::MediumInterface;
use pbrt::core::paramset::ParamSet;
use pbrt::core::pbrt::{Float, Spectrum};
use pbrt::core::primitive::{GeometricPrimitive, Primitive, TransformedPrimitive};
use pbrt::core::sampler::Sampler;
use pbrt::core::scene::Scene;
use pbrt::core::shape::Shape;
use pbrt::core::transform::{AnimatedTransform, Matrix4x4, Transform};
use pbrt::filters::gaussian::GaussianFilter;
use pbrt::integrators::ao::AOIntegrator;
use pbrt::integrators::path::PathIntegrator;
use pbrt::integrators::render;
use pbrt::lights::diffuse::DiffuseAreaLight;
use pbrt::lights::point::PointLight;
use pbrt::materials::matte::MatteMaterial;
use pbrt::samplers::sobol::SobolSampler;
use pbrt::shapes::disk::Disk;
use pbrt::shapes::triangle::{Triangle, TriangleMesh};
use pbrt::textures::constant::ConstantTexture;
// std
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

#[derive(Parser)]
#[grammar = "../examples/ass.pest"]
struct AssParser;

// TransformSet (copied from api.rs)

#[derive(Debug, Default, Copy, Clone)]
pub struct TransformSet {
    pub t: [Transform; 2],
}

impl TransformSet {
    pub fn is_animated(&self) -> bool {
        // for (int i = 0; i < MaxTransforms - 1; ++i)
        //     if (t[i] != t[i + 1]) return true;
        // return false;

        // we have only 2 transforms
        if self.t[0] != self.t[1] {
            true
        } else {
            false
        }
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn print_version(program: &str) {
    println!("{} {}", program, VERSION);
}

fn strip_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let v: Vec<&str> = input.lines().map(str::trim).collect();
    for line in v {
        if let Some(_found) = line.find('#') {
            let v2: Vec<&str> = line.split('#').collect();
            let stripped_line = v2[0];
            output.push_str(stripped_line);
            output.push_str("\n");
        } else {
            output.push_str(line);
            output.push_str("\n");
        }
    }
    output
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("i", "", "parse an input file", "FILE");
    opts.optflag("v", "version", "print version number");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("i") {
        // default values
        let mut node_name: String = String::from(""); // no default name
        let mut filter_name: String = String::from("box");
        let mut filter_width: Float = 2.0;
        let mut render_camera: String = String::from(""); // no default name
        let mut mesh: String = String::from(""); // no default name
        let mut camera_name: String = String::from("perspective");
        let mut fov: Float = 90.0; // read persp_camera.fov
        let mut intensity: Float = 1.0; // read mesh_light.intensity
        let mut radius: Float = 0.5; // read [cylinder, disk, sphere].radius
        let mut color: Spectrum = Spectrum::new(1.0 as Float);
        let mut animated_cam_to_world: AnimatedTransform = AnimatedTransform::default();
        let mut xres: i32 = 1280; // read options.xres
        let mut yres: i32 = 720; // read options.yres
        let mut max_depth: i32 = 5; // read options.GI_total_depth
        let mut samples: i32 = 1; // read mesh_light.samples
        let mut cur_transform: Transform = Transform::default();
        let mut obj_to_world: Transform = Transform::default();
        let mut world_to_obj: Transform = Transform::default();
        let mut nsides: Vec<u32> = Vec::new();
        let mut p_ws: Vec<Point3f> = Vec::new();
        let mut p_ws_len: usize = 0;
        let mut vi: Vec<u32> = Vec::new();
        let mut primitives: Vec<Arc<Primitive + Sync + Send>> = Vec::new();
        let mut lights: Vec<Arc<Light + Sync + Send>> = Vec::new();
        let mut named_primitives: HashMap<String, Vec<Arc<GeometricPrimitive>>> = HashMap::new();
        // input (.ass) file
        let infile = matches.opt_str("i");
        match infile {
            Some(x) => {
                println!("FILE = {}", x);
                let f = File::open(x.clone()).unwrap();
                let ip: &Path = Path::new(x.as_str());
                if ip.is_relative() {
                    let cp: PathBuf = env::current_dir().unwrap();
                    let pb: PathBuf = cp.join(ip);
                    let search_directory: &Path = pb.as_path().parent().unwrap();
                    println!("search_directory is {}", search_directory.display());
                }
                let mut reader = BufReader::new(f);
                let mut str_buf: String = String::default();
                let num_bytes = reader.read_to_string(&mut str_buf);
                if num_bytes.is_ok() {
                    let n_bytes = num_bytes.unwrap();
                    println!("{} bytes read", n_bytes);
                }
                // parser
                let pairs =
                    AssParser::parse(Rule::ass, &str_buf).unwrap_or_else(|e| panic!("{}", e));
                // let tokens: Vec<_> = pairs.flatten().tokens().collect();
                // println!("{} pairs", tokens.len());
                for pair in pairs {
                    let span = pair.clone().into_span();
                    // println!("Rule:    {:?}", pair.as_rule());
                    // println!("Span:    {:?}", span);
                    // println!("Text:    {}", span.as_str());
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::ident => {
                                let node_type = inner_pair.clone().into_span().as_str();
                                print!("{} {{", node_type);
                                let stripped = strip_comments(span.as_str());
                                let mut iter = stripped.split_whitespace().peekable();
                                loop {
                                    if let Some(next) = iter.next() {
                                        if next != String::from("}") {
                                            // for all nodes
                                            if next == String::from("name") {
                                                if let Some(name) = iter.next() {
                                                    node_name = name.to_string();
                                                    print!(" {} {} ", next, node_name);
                                                }
                                            } else if next == String::from("matrix") {
                                                let mut elems: Vec<Float> = Vec::new();
                                                let expected: u32 = 16;
                                                for _i in 0..expected {
                                                    if let Some(elem_str) = iter.next() {
                                                        let elem: f32 = f32::from_str(elem_str).unwrap();
                                                        elems.push(elem as Float);
                                                    }
                                                }
                                                print!("\n matrix ... ");
                                                // print!("\n {:?}", elems);
                                                let m00: Float = elems[0];
                                                let m01: Float = elems[1];
                                                let m02: Float = elems[2];
                                                let m03: Float = elems[3];
                                                let m10: Float = elems[4];
                                                let m11: Float = elems[5];
                                                let m12: Float = elems[6];
                                                let m13: Float = elems[7];
                                                let m20: Float = elems[8];
                                                let m21: Float = elems[9];
                                                let m22: Float = elems[10];
                                                let m23: Float = elems[11];
                                                let m30: Float = elems[12];
                                                let m31: Float = elems[13];
                                                let m32: Float = elems[14];
                                                let m33: Float = elems[15];
                                                cur_transform = Transform::new(
                                                    m00, m10, m20, m30, m01, m11, m21, m31, m02,
                                                    m12, m22, m32, m03, m13, m23, m33,
                                                );
                                                // print!("\n {:?}", cur_transform);
                                                obj_to_world = Transform {
                                                    m: cur_transform.m,
                                                    m_inv: cur_transform.m_inv,
                                                };
                                                world_to_obj = Transform {
                                                    m: cur_transform.m_inv,
                                                    m_inv: cur_transform.m,
                                                };
                                                if node_type == String::from("persp_camera")
                                                    && node_name == render_camera
                                                {
                                                    let transform_start_time: Float = 0.0;
                                                    let transform_end_time: Float = 1.0;
                                                    let scale: Transform = Transform::scale(1.0 as Float,
                                                                                            1.0 as Float,
                                                                                            -1.0 as Float);
                                                    cur_transform = cur_transform * scale;
                                                    animated_cam_to_world = AnimatedTransform::new(
                                                        &cur_transform,
                                                        transform_start_time,
                                                        &cur_transform,
                                                        transform_end_time,
                                                    );
                                                }
                                            }
                                            // by node type
                                            if node_type == String::from("options") {
                                                if next == String::from("xres") {
                                                    if let Some(xres_str) = iter.next() {
                                                        xres = i32::from_str(xres_str).unwrap();
                                                        print!("\n xres {} ", xres);
                                                    }
                                                } else if next == String::from("yres") {
                                                    if let Some(yres_str) = iter.next() {
                                                        yres = i32::from_str(yres_str).unwrap();
                                                        print!("\n yres {} ", yres);
                                                    }
                                                } else if next == String::from("camera") {
                                                    if let Some(camera_str) = iter.next() {
                                                        // strip surrounding double quotes
                                                        let v: Vec<&str> = camera_str.split('"').collect();
                                                        render_camera = v[1].to_string();
                                                        print!("\n camera {:?} ", render_camera);
                                                    }
                                                } else if next == String::from("GI_total_depth") {
                                                    if let Some(max_depth_str) = iter.next() {
                                                        max_depth =
                                                            i32::from_str(max_depth_str).unwrap();
                                                        print!("\n GI_total_depth {} ", max_depth);
                                                    }
                                                }
                                            } else if node_type == String::from("persp_camera")
                                                && node_name == render_camera
                                            {
                                                camera_name = String::from("perspective");
                                                if next == String::from("fov") {
                                                    if let Some(fov_str) = iter.next() {
                                                        fov = f32::from_str(fov_str).unwrap();
                                                        print!("\n fov {} ", fov);
                                                    }
                                                }
                                            } else if node_type == String::from("gaussian_filter") {
                                                filter_name = String::from("gaussian");
                                                if next == String::from("width") {
                                                    if let Some(filter_width_str) = iter.next() {
                                                        filter_width =
                                                            f32::from_str(filter_width_str)
                                                                .unwrap();
                                                        print!("\n filter_width {} ", filter_width);
                                                    }
                                                }
                                            } else if node_type == String::from("mesh_light") {
                                                if next == String::from("intensity") {
                                                    if let Some(intensity_str) = iter.next() {
                                                        intensity =
                                                            f32::from_str(intensity_str).unwrap();
                                                        print!("\n intensity {} ", intensity);
                                                    }
                                                } else if next == String::from("color") {
                                                    let mut color_r: Float = 0.0;
                                                    let mut color_g: Float = 0.0;
                                                    let mut color_b: Float = 0.0;
                                                    if let Some(color_str) = iter.next() {
                                                        color_r = f32::from_str(color_str).unwrap();
                                                    }
                                                    if let Some(color_str) = iter.next() {
                                                        color_g = f32::from_str(color_str).unwrap();
                                                    }
                                                    if let Some(color_str) = iter.next() {
                                                        color_b = f32::from_str(color_str).unwrap();
                                                    }
                                                    color =
                                                        Spectrum::rgb(color_r, color_g, color_b);
                                                    print!(
                                                        "\n color {} {} {} ",
                                                        color_r, color_g, color_b
                                                    );
                                                } else if next == String::from("samples") {
                                                    if let Some(samples_str) = iter.next() {
                                                        samples =
                                                            i32::from_str(samples_str).unwrap();
                                                        print!("\n samples {} ", samples);
                                                    }
                                                } else if next == String::from("mesh") {
                                                    if let Some(mesh_str) = iter.next() {
                                                        // strip surrounding double quotes
                                                        let v: Vec<&str> = mesh_str.split('"').collect();
                                                        mesh = v[1].to_string();
                                                        print!("\n mesh {:?} ", mesh);
                                                    }
                                                }
                                            } else if node_type == String::from("polymesh") {
                                                if next == String::from("vlist") {
                                                    // parameter_name: vlist
                                                    // <num_elements>
                                                    // <num_motionblur_keys>
                                                    // <data_type>: VECTOR
                                                    // <elem1> <elem2>
                                                    // <elem3> <elem4>
                                                    // ...
                                                    let mut num_elements: u32 = 0;
                                                    let mut num_motionblur_keys: u32 = 1;
                                                    let data_type: String = String::from("VECTOR");
                                                    let mut elems: Vec<Float> = Vec::new();
                                                    if let Some(num_elements_str) = iter.next() {
                                                        num_elements =
                                                            u32::from_str(num_elements_str)
                                                                .unwrap();
                                                        if let Some(num_motionblur_keys_str) =
                                                            iter.next()
                                                        {
                                                            num_motionblur_keys =
                                                                u32::from_str(num_motionblur_keys_str).unwrap();
                                                            if let Some(data_type_str) = iter.next()
                                                            {
                                                                if data_type_str != data_type {
                                                                    panic!(
                                                                        "ERROR: {} expected ...",
                                                                        data_type
                                                                    );
                                                                } else {
                                                                    let expected: u32 = num_elements * num_motionblur_keys * 3;
                                                                    for _i in 0..expected {
                                                                        if let Some(elem_str) =
                                                                            iter.next()
                                                                        {
                                                                            let elem: f32 =
                                                                                f32::from_str(elem_str)
                                                                                .unwrap();
                                                                            elems.push(
                                                                                elem as Float,
                                                                            );
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    print!(
                                                        "\n vlist {} {} VECTOR ... ",
                                                        num_elements, num_motionblur_keys
                                                    );
                                                    // print!("\n {:?}", elems);
                                                    // TriangleMesh
                                                    let mut x: Float = 0.0;
                                                    let mut y: Float = 0.0;
                                                    let mut z;
                                                    let mut p: Vec<Point3f> = Vec::new();
                                                    for i in 0..elems.len() {
                                                        if i % 3 == 0 {
                                                            x = elems[i];
                                                        } else if i % 3 == 1 {
                                                            y = elems[i];
                                                        } else {
                                                            // i % 3 == 2
                                                            z = elems[i];
                                                            // store as Point3f
                                                            p.push(Point3f { x: x, y: y, z: z });
                                                        }
                                                    }
                                                    // transform mesh vertices to world space
                                                    p_ws = Vec::new();
                                                    let n_vertices: usize = p.len();
                                                    for i in 0..n_vertices {
                                                        p_ws.push(
                                                            obj_to_world.transform_point(&p[i]),
                                                        );
                                                    }
                                                    p_ws_len = p_ws.len();
                                                // print info
                                                // println!("");
                                                // for point in p {
                                                //     println!(" {:?}", point);
                                                // }
                                                } else if next == String::from("nsides") {
                                                    nsides = Vec::new();
                                                    loop {
                                                        let mut is_int: bool = false;
                                                        // check if next string can be converted to u32
                                                        if let Some(ref check_for_int_str) =
                                                            iter.peek()
                                                        {
                                                            if u32::from_str(check_for_int_str)
                                                                .is_ok()
                                                            {
                                                                is_int = true;
                                                            } else {
                                                                // if not ... break the loop
                                                                break;
                                                            }
                                                        }
                                                        // if we can convert use next()
                                                        if is_int {
                                                            if let Some(nside_str) = iter.next() {
                                                                let nside: u32 = u32::from_str(nside_str).unwrap();
                                                                nsides.push(nside);
                                                            }
                                                        }
                                                    }
                                                    let mut followed_by_uint: bool = false;
                                                    // check if next string is 'UINT' (or not)
                                                    if let Some(check_for_uint_str) = iter.peek() {
                                                        if **check_for_uint_str
                                                            == String::from("UINT")
                                                        {
                                                            followed_by_uint = true;
                                                        }
                                                    }
                                                    if followed_by_uint {
                                                        // skip next (we checked already)
                                                        iter.next();
                                                        let num_elements = nsides[0];
                                                        let num_motionblur_keys = nsides[1];
                                                        print!(
                                                            "\n nsides {} {} UINT ... ",
                                                            num_elements, num_motionblur_keys
                                                        );
                                                        let expected: u32 = num_elements * num_motionblur_keys;
                                                        nsides = Vec::new();
                                                        for _i in 0..expected {
                                                            if let Some(nside_str) = iter.next() {
                                                                let nside: u32 = u32::from_str(nside_str).unwrap();
                                                                nsides.push(nside);
                                                            }
                                                        }
                                                    } else {
                                                        print!("\n nsides ... ");
                                                    }
                                                // print!("\n {:?} ", nsides);
                                                } else if next == String::from("vidxs") {
                                                    // parameter_name: vidxs
                                                    // <num_elements>
                                                    // <num_motionblur_keys>
                                                    // <data_type>: UINT
                                                    // <elem1> <elem2>
                                                    // <elem3> <elem4>
                                                    // ...
                                                    let mut num_elements: u32 = 0;
                                                    let mut num_motionblur_keys: u32 = 1;
                                                    let data_type: String = String::from("UINT");
                                                    vi = Vec::new();
                                                    if let Some(num_elements_str) = iter.next() {
                                                        num_elements =
                                                            u32::from_str(num_elements_str)
                                                                .unwrap();
                                                        if let Some(num_motionblur_keys_str) =
                                                            iter.next()
                                                        {
                                                            num_motionblur_keys =
                                                                u32::from_str(num_motionblur_keys_str).unwrap();
                                                            if let Some(data_type_str) = iter.next()
                                                            {
                                                                if data_type_str != data_type {
                                                                    panic!(
                                                                        "ERROR: {} expected ...",
                                                                        data_type
                                                                    );
                                                                } else {
                                                                    let expected: u32 = num_elements * num_motionblur_keys;
                                                                    for _i in 0..expected {
                                                                        if let Some(elem_str) =
                                                                            iter.next()
                                                                        {
                                                                            let elem: u32 =
                                                                                u32::from_str(elem_str)
                                                                                .unwrap();
                                                                            vi.push(elem);
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    print!(
                                                        "\n vidxs {} {} UINT ... ",
                                                        num_elements, num_motionblur_keys
                                                    );
                                                    // print!("\n {:?} ", vi);
                                                }
                                            } else if node_type == String::from("disk") {
                                                if next == String::from("radius") {
                                                    if let Some(radius_str) = iter.next() {
                                                        radius = 0.5; // reset
                                                        radius = f32::from_str(radius_str).unwrap();
                                                        print!("\n radius {} ", radius);
                                                    }
                                                }
                                            }
                                        } else {
                                            println!("}}");
                                            if node_type == String::from("mesh_light") {
                                                match named_primitives.get_mut(mesh.as_str()) {
                                                    Some(prims) => {
                                                        // for i in 0..prims.len() {
                                                        //     let mut prim = &mut prims[i];
                                                        for prim in prims.iter_mut() {
                                                            let shape = prim.shape.clone();
                                                            let geo_prim_opt = Arc::get_mut(prim);
                                                            let mi: MediumInterface =
                                                                MediumInterface::default();
                                                            let l_emit: Spectrum = color * intensity;
                                                            let two_sided: bool = false;
                                                            let area_light: Arc<DiffuseAreaLight> =
                                                                Arc::new(DiffuseAreaLight::new(
                                                                    &cur_transform,
                                                                    &mi,
                                                                    &l_emit,
                                                                    samples,
                                                                    shape,
                                                                    two_sided,
                                                                ));
                                                            lights.push(area_light.clone());
                                                            // pointer from prim to area light
                                                            if geo_prim_opt.is_some() {
                                                                let mut geo_prim = geo_prim_opt.unwrap();
                                                                geo_prim.area_light =
                                                                    Some(area_light.clone());
                                                            } else {
                                                                println!("WARNING: no pointer from prim to area light");
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        panic!(
                                                            "ERROR: mesh_light({:?}) without mesh",
                                                            mesh
                                                        );
                                                    }
                                                }
                                            } else if node_type == String::from("point_light") {
                                                let mi: MediumInterface = MediumInterface::default();
                                                let point_light = Arc::new(PointLight::new(
                                                    &cur_transform,
                                                    &mi,
                                                    &(color * intensity),
                                                ));
                                                lights.push(point_light);
                                            } else if node_type == String::from("polymesh") {
                                                // make sure there are no out of-bounds vertex indices
                                                for i in 0..vi.len() {
                                                    if vi[i] as usize >= p_ws_len {
                                                        panic!(
                                                            "trianglemesh has out of-bounds vertex index {} ({} \"P\" values were given)",
                                                            vi[i],
                                                            p_ws_len
                                                        );
                                                    }
                                                }
                                                // convert quads to triangles
                                                let mut vi_tri: Vec<u32> = Vec::new();
                                                let mut count: usize = 0;
                                                for i in 0..nsides.len() {
                                                    let nside = nsides[i];
                                                    if nside == 3 {
                                                        // triangle
                                                        vi_tri.push(vi[count]);
                                                        count += 1;
                                                        vi_tri.push(vi[count]);
                                                        count += 1;
                                                        vi_tri.push(vi[count]);
                                                        count += 1;
                                                    } else if nside == 4 {
                                                        // quad gets split into 2 triangles
                                                        vi_tri.push(vi[count]);
                                                        vi_tri.push(vi[count + 1]);
                                                        vi_tri.push(vi[count + 2]);
                                                        vi_tri.push(vi[count]);
                                                        vi_tri.push(vi[count + 2]);
                                                        vi_tri.push(vi[count + 3]);
                                                        count += 4;
                                                    } else {
                                                        panic!(
                                                            "{}-sided poygons are not supported",
                                                            nside
                                                        );
                                                    }
                                                }
                                                // TriangleMesh
                                                let mut shapes: Vec<Arc<Shape + Send + Sync>> = Vec::new();
                                                let mut materials: Vec<Option<Arc<Material + Send + Sync>>> = Vec::new();
                                                let s_ws: Vec<Vector3f> = Vec::new();
                                                let n_ws: Vec<Normal3f> = Vec::new();
                                                let uvs: Vec<Point2f> = Vec::new();
                                                // vertex indices are expected as usize, not u32
                                                let mut vertex_indices: Vec<usize> = Vec::new();
                                                for i in 0..vi_tri.len() {
                                                    vertex_indices.push(vi_tri[i] as usize);
                                                }
                                                let mesh = Arc::new(TriangleMesh::new(
                                                    obj_to_world,
                                                    world_to_obj,
                                                    false,            // reverse_orientation,
                                                    false,            // transform_swaps_handedness
                                                    vi_tri.len() / 3, // n_triangles
                                                    vertex_indices,
                                                    p_ws_len,
                                                    p_ws.clone(), // in world space
                                                    s_ws,         // in world space
                                                    n_ws,         // in world space
                                                    uvs,
                                                ));
                                                let kd = Arc::new(ConstantTexture::new(
                                                    Spectrum::new(0.5),
                                                ));
                                                let sigma =
                                                    Arc::new(ConstantTexture::new(0.0 as Float));
                                                let matte = Arc::new(MatteMaterial::new(kd, sigma));
                                                let mtl: Option<Arc<Material + Send + Sync>> = Some(matte);
                                                for id in 0..mesh.n_triangles {
                                                    let triangle = Arc::new(Triangle::new(
                                                        mesh.object_to_world,
                                                        mesh.world_to_object,
                                                        mesh.reverse_orientation,
                                                        mesh.clone(),
                                                        id,
                                                    ));
                                                    shapes.push(triangle.clone());
                                                    materials.push(mtl.clone());
                                                }
                                                let mi: MediumInterface = MediumInterface::default();
                                                let mut prims: Vec<Arc<GeometricPrimitive>> = Vec::new();
                                                for i in 0..shapes.len() {
                                                    let shape = &shapes[i];
                                                    let material = &materials[i];
                                                    let geo_prim =
                                                        Arc::new(GeometricPrimitive::new(
                                                            shape.clone(),
                                                            material.clone(),
                                                            None,
                                                            Some(Arc::new(mi.clone())),
                                                        ));
                                                    prims.push(geo_prim.clone());
                                                }
                                                named_primitives.insert(node_name.clone(), prims);
                                            } else if node_type == String::from("disk") {
                                                let mut shapes: Vec<Arc<Shape + Send + Sync>> = Vec::new();
                                                let mut materials: Vec<Option<Arc<Material + Send + Sync>>> = Vec::new();
                                                let kd = Arc::new(ConstantTexture::new(
                                                    Spectrum::new(0.0),
                                                ));
                                                let sigma =
                                                    Arc::new(ConstantTexture::new(0.0 as Float));
                                                let matte = Arc::new(MatteMaterial::new(kd, sigma));
                                                let mtl: Option<Arc<Material + Send + Sync>> = Some(matte);
                                                let disk = Arc::new(Disk::new(
                                                    obj_to_world,
                                                    world_to_obj,
                                                    false,
                                                    false,
                                                    0.0 as Float, // height
                                                    radius,
                                                    0.0 as Float,   // inner_radius
                                                    360.0 as Float, // phi_max
                                                ));
                                                shapes.push(disk.clone());
                                                materials.push(mtl.clone());
                                                let mi: MediumInterface = MediumInterface::default();
                                                let mut prims: Vec<Arc<GeometricPrimitive>> = Vec::new();
                                                for i in 0..shapes.len() {
                                                    let shape = &shapes[i];
                                                    let material = &materials[i];
                                                    let geo_prim =
                                                        Arc::new(GeometricPrimitive::new(
                                                            shape.clone(),
                                                            material.clone(),
                                                            None,
                                                            Some(Arc::new(mi.clone())),
                                                        ));
                                                    prims.push(geo_prim.clone());
                                                }
                                                named_primitives.insert(node_name.clone(), prims);
                                            }
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => println!("TODO: {:?}", inner_pair.as_rule()),
                        }
                    }
                }
            }
            None => panic!("No input file name."),
        }
        println!("render_camera = {:?} ", render_camera);
        println!("fov = {:?} ", fov);
        println!("filter_name = {:?}", filter_name);
        println!("filter_width = {:?}", filter_width);
        println!("max_depth = {:?}", max_depth);
        for value in named_primitives.values() {
            for prim in value {
                primitives.push(prim.clone());
            }
        }
        println!("number of primitives = {:?}", primitives.len());
        // MakeFilter
        let mut some_filter: Option<Arc<Filter + Sync + Send>> = None;
        if filter_name == String::from("box") {
            println!("TODO: CreateBoxFilter");
        } else if filter_name == String::from("gaussian") {
            let mut filter_params: ParamSet = ParamSet::default();
            filter_params.add_float(String::from("xwidth"), filter_width);
            filter_params.add_float(String::from("ywidth"), filter_width);
            some_filter = Some(GaussianFilter::create(&filter_params));
        } else if filter_name == String::from("mitchell") {
            println!("TODO: CreateMitchellFilter");
        } else if filter_name == String::from("sinc") {
            println!("TODO: CreateSincFilter");
        } else if filter_name == String::from("triangle") {
            println!("TODO: CreateTriangleFilter");
        } else {
            panic!("Filter \"{}\" unknown.", filter_name);
        }
        // MakeFilm
        let resolution: Point2i = Point2i { x: xres, y: yres };
        println!("resolution = {:?}", resolution);
        if let Some(filter) = some_filter {
            let crop: Bounds2f = Bounds2f {
                p_min: Point2f { x: 0.0, y: 0.0 },
                p_max: Point2f { x: 1.0, y: 1.0 },
            };
            let diagonal: Float = 35.0;
            let scale: Float = 1.0;
            let max_sample_luminance: Float = std::f32::INFINITY;
            let film: Arc<Film> = Arc::new(Film::new(
                resolution,
                crop,
                filter,
                diagonal,
                String::from(""),
                scale,
                max_sample_luminance,
            ));
            // MakeCamera
            let mut some_camera: Option<Arc<Camera + Sync + Send>> = None;
            let mut medium_interface: MediumInterface = MediumInterface::default();
            if camera_name == String::from("perspective") {
                let mut camera_params: ParamSet = ParamSet::default();
                camera_params.add_float(String::from("fov"), fov);
                let camera: Arc<Camera + Send + Sync> = PerspectiveCamera::create(
                    &camera_params,
                    animated_cam_to_world,
                    film,
                    medium_interface.outside,
                );
                some_camera = Some(camera);
            } else if camera_name == String::from("orthographic") {
                println!("TODO: CreateOrthographicCamera");
            } else if camera_name == String::from("realistic") {
                println!("TODO: CreateRealisticCamera");
            } else if camera_name == String::from("environment") {
                println!("TODO: CreateEnvironmentCamera");
            } else {
                panic!("Camera \"{}\" unknown.", camera_name);
            }
            if let Some(camera) = some_camera {
                // MakeSampler
                let mut some_sampler: Option<Box<Sampler + Sync + Send>> = None;
                // use SobolSampler for now
                let nsamp: i64 = 16; // TODO: something from .ass file
                let sample_bounds: Bounds2i = camera.get_film().get_sample_bounds();
                let sampler = Box::new(SobolSampler::new(nsamp, sample_bounds));
                some_sampler = Some(sampler);
                if let Some(mut sampler) = some_sampler {
                    // MakeIntegrator
                    let mut some_integrator: Option<
                        Box<SamplerIntegrator + Sync + Send>,
                    > = None;
                    // CreateAOIntegrator
                    // let pixel_bounds: Bounds2i = camera.get_film().get_sample_bounds();
                    // let cos_sample: bool = true;
                    // let n_samples: i32 = 64;
                    // let integrator =
                    //     Box::new(AOIntegrator::new(cos_sample, n_samples, pixel_bounds));
                    // CreatePathIntegrator
                    let pixel_bounds: Bounds2i = camera.get_film().get_sample_bounds();
                    let rr_threshold: Float = 1.0;
                    let light_strategy: String = String::from("spatial");
                    let integrator = Box::new(PathIntegrator::new(
                        max_depth as u32,
                        pixel_bounds,
                        rr_threshold,
                        light_strategy,
                    ));
                    some_integrator = Some(integrator);
                    if let Some(mut integrator) = some_integrator {
                        // MakeIntegrator
                        if lights.is_empty() {
                            // warn if no light sources are defined
                            print!("WARNING: No light sources defined in scene; ");
                            println!("rendering a black image.");
                        }
                        if !primitives.is_empty() {
                            let split_method = SplitMethod::SAH;
                            let max_prims_in_node: i32 = 4;
                            let accelerator = Arc::new(BVHAccel::new(
                                primitives.clone(),
                                max_prims_in_node as usize,
                                split_method,
                            ));
                            let scene: Scene = Scene::new(accelerator.clone(), lights.clone());
                            let num_threads: u8 = num_cpus::get() as u8;
                            render(
                                &scene,
                                &camera.clone(),
                                &mut sampler,
                                &mut integrator,
                                num_threads,
                            );
                        } else {
                            print!("WARNING: No primitives defined in scene; ");
                            println!("no need to render anything.");
                        }
                    }
                }
            }
        }
        return;
    } else if matches.opt_present("v") {
        print_version(&program);
        return;
    } else {
        print_usage(&program, opts);
        return;
    }
}
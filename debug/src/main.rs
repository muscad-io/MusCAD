#[macro_use]
extern crate muscad_core;
extern crate muscad_gluon;

use muscad_core::sealed as muscad;

#[allow(unused)]
use muscad::*;

#[allow(unused)]
use muscad_gluon::gluon;

#[allow(unused)]
use std::{cell::RefCell, fs::File, io::Write, rc::Rc};

mod intf;

#[allow(unused)]
fn run() {
    //run_gluon_script();

    run_boolean();
}

#[allow(unused)]
fn run_boolean() {
    let a = gen_cude(true);
    //let b = gen_cude(false);
    let b = gen_torus(20, 20, 0.9, 0.5);

    let mut solver = muscad::BooleanSolver::<f64>::new();
    setup_hooks(&mut solver);

    solver.mode = muscad::BooleanMode::AMinusB;
    let c = solver.eval(&a, &b);

    save_file(&[a, b, c]);
}

#[allow(unused)]
unsafe fn serialize<T: serde::Serialize>(c: &RefCell<T>) -> String {
    serde_json::to_string(RefCell::as_ptr(c).as_ref().unwrap()).unwrap()
}

#[allow(unused)]
pub fn save_file(models: &[muscad::Model<f64>]) {
    let mut file = File::create(concat!(env!("CARGO_MANIFEST_DIR"), "/data.json")).unwrap();
    file.write_all(serde_json::to_string(&models).unwrap().as_bytes())
        .unwrap();
}

#[allow(unused)]
fn setup_hooks<T: muscad::Float + serde::Serialize>(solver: &mut muscad::BooleanSolver<T>) {
    set_hook!(solver, classify_group1, |a, b, c, d| {
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", d);
    });
    set_hook!(solver, classify_group2, |a| {
        println!("{:?}", a);
    });
    set_hook!(solver, classify_group3, |a, b| {
        println!("{:?}", a);
        println!("{:?}", b);
    });
    set_hook!(solver, new_face1, |d| {
        println!("{:?}", d);
    });
    set_hook!(solver, new_face2, |a, b| {
        println!("{:?}", a);
        println!("{:?}", b);
    });
    set_hook!(solver, new_face3, |d| {
        println!("{:?}", d);
    });

    set_hook!(solver, intersect_face_edge, |a, b, c, d| {
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", d);
    });

    set_hook!(solver, add_inner_loop, |a, b, c| {
        use muscad::AsSegment;
        println!("{:?}", a.to_p1());
        println!("{:?}", b);
        println!("{:?}", c);
    });
}

#[allow(unused)]
fn gen_torus(slices: usize, rings: usize, rad1: f64, rad2: f64) -> muscad::Model<f64> {
    let mut data = muscad::Model::new();
    let mut vs = vec![];
    let tau = f64::PI * 2.0;

    for i in 0..slices {
        let a1 = i as f64 * tau / slices as f64;
        let dy = a1.cos();
        let dx = a1.sin();
        for j in 0..rings {
            let a2 = j as f64 * tau / rings as f64;
            let x = dx * (rad1 + a2.cos() * rad2);
            let y = dy * (rad1 + a2.cos() * rad2);
            let z = a2.sin() * rad2;
            vs.push([x, y, z]);
        }
    }

    let v = |i, j| vs[i * rings + j];

    for i in 0..slices {
        let i2 = (i + 1) % slices;
        for j in 0..rings {
            let j2 = (j + 1) % rings;
            data.add_face_unchecked(&[v(i, j), v(i, j2), v(i2, j2), v(i2, j)]);
        }
    }

    data
}

#[allow(unused)]
fn gen_cude(is_big: bool) -> muscad::Model<f64> {
    let v = if is_big {
        vec![
            [1.52, 0.46, 1.69],
            [1.52, 1.46, 1.69],
            [0.52, 1.46, 1.69],
            [0.52, 0.46, 1.69],
            [1.52, 0.46, 0.69],
            [1.52, 1.46, 0.69],
            [0.52, 0.46, 0.69],
            [0.52, 1.46, 0.69],
        ]
    } else {
        vec![
            [1.52, -0.54, 0.69],
            [1.52, 0.46, 0.69],
            [0.52, 0.46, 0.69],
            [0.52, -0.54, 0.69],
            [1.52, -0.54, -0.31],
            [1.52, 0.46, -0.31],
            [0.52, -0.54, -0.31],
            [0.52, 0.46, -0.31],
        ]
    };

    let mut d = muscad::Model::new();
    d.add_face_unchecked(&[&v[0], &v[1], &v[2], &v[3]]);
    d.add_face_unchecked(&[&v[4], &v[5], &v[1], &v[0]]);
    d.add_face_unchecked(&[&v[6], &v[4], &v[0], &v[3]]);
    d.add_face_unchecked(&[&v[7], &v[6], &v[3], &v[2]]);
    d.add_face_unchecked(&[&v[5], &v[7], &v[2], &v[1]]);
    d.add_face_unchecked(&[&v[5], &v[4], &v[6], &v[7]]);

    d
}

#[allow(unused)]
fn run_gluon_script() {
    use muscad_gluon::ThreadExt;
    let vm = muscad_gluon::vm::create_vm();
    gluon::import::add_extern_module(&vm, "js_interface", intf::load);
    vm.run_io(true);
    vm.load_script("tmp.glu", include_str!("./tmp.glu"))
        .unwrap_or_else(|e| eprintln!("{}", e));
}

use std::time::Instant;

#[allow(unused)]
fn main() {
    let now = Instant::now();

    run();

    println!("Done in {}ms", now.elapsed().as_millis());
}

#[allow(unused)]
fn rand() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::BuildHasher;
    use std::hash::Hasher;
    let v = RandomState::new().build_hasher().finish();

    v
}

#[allow(unused)]
fn rand_rgb() -> (u8, u8, u8) {
    let arr = unsafe { std::mem::transmute::<_, [u8; 8]>(rand()) };

    (arr[5], arr[6], arr[7])
}

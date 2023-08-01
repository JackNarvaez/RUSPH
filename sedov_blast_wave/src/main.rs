// Sedov Blast Wave

use std::{
    fs::File,
    io::Write,
    error::Error,
    process,
    time::Instant,
};

use sphfunctions;
use datafunctions;

use tree_algorithm::BuildTree;

use structures::{
    Particle,
    Node,
    Pointer,
};

use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn Error>> {

    // File's information
    let path_source = "./Data/initial_distribution/sedov_blast_wave.csv";
    let input_file = "./sedov_blast_wave/input";

    let input: Vec<f64> = datafunctions::read_input(input_file);
    let mut particles :Vec<Particle> = Vec::new();
    if let Err(err) = datafunctions::read_data(path_source, &mut particles) {
        println!("{}", err);
        process::exit(1);
    }
    let particles_ptr = Pointer(particles.as_mut_ptr());

    // Simulation's parameters
    let t0:f64 = input[12]; // Initial time
    let tf:f64 = input[13]; // Final time
    let mut t:f64 = t0; // Time
    let n : usize = particles.len(); // Number of particles
    let mut time_file = File::create("./Data/results/sedov_blast_wave/Time.txt").expect("creation failed"); // Save time steps
    
    // System's parameters
    let eta :f64 = input[0]; // Dimensionless constant related to the ratio of smoothing length
    let d: i32 = input[1] as i32; // Dimension of the system
    let gamma:f64 = input[2];  // Gamma factor (heat capacity ratio)
    //___________________________________________________________
    let sigma :f64 = 1./PI; // Normalization's constant of kernel
    let rkern: f64 = 2.;
    //___________________________________________________________
    let wd:f64 = input[6]; // Width
    let lg:f64 = input[7]; // Longth
    let hg:f64 = input[8]; // Heigth
    let x0:f64 = input[3]; // Initial x
    let y0:f64 = input[4]; // Initial y
    let z0:f64 = input[5]; // Initial y
    let rho0:f64 = input[10]; // Density
    let dm :f64 = rho0*(wd*lg*hg)/n as f64; // Particles' mass
    
    for ii in 0..n {
        particles[ii].rho = sphfunctions::density_by_smoothing_length(dm, particles[ii].h, eta, d);
    }
    
    // Save initial information
    time_file.write((t.to_string() + &"\n").as_bytes()).expect("write failed");
    if let Err(err) = datafunctions::save_data(&(String::from("./Data/results/sedov_blast_wave/initial.csv")), &particles){
        println!("{}", err);
        process::exit(1);
    }

    // Tree's parameters
    let s_ : i32 = input[19] as i32;
    let alpha_ : f64 = input[20];
    let beta_ : f64 = input[21];
    let mut tree : Node = <Node as BuildTree>::new(n as i32, x0, y0, z0, wd, lg, hg);
    
    let mut dt :f64 = input[14]; // Time step
    let mut it: u32 = 0; // Time iterations
    let it_save: u32 = input[15] as u32; // Frequency of data saving

    // Main loop
    let start = Instant::now(); // Runing time
    while t < tf  {
        sphfunctions::predictor_kdk_integrator(&mut particles, dt, dm, sphfunctions::eos_ideal_gas, sphfunctions::sound_speed_ideal_gas, gamma,
                                       sphfunctions::dwdh, sphfunctions::f_cubic_kernel, sphfunctions::dfdq_cubic_kernel, sigma, rkern,
                                       d, eta, &mut tree, s_, alpha_, beta_, n, particles_ptr,
                                       sphfunctions::mon97_art_vis,
                                       sphfunctions::body_forces_null, 0.0, 0.0, false,
                                       sphfunctions::periodic_boundary, wd, lg, hg, x0, y0, z0);
        dt = sphfunctions::time_step_mon(&particles, n, gamma, rkern, d, wd, lg, hg, x0, y0, z0, &mut tree, s_, sphfunctions::sound_speed_ideal_gas_u);
        tree.restart(n);
        t += dt;
        if (it%it_save) == 0 {
            time_file.write((t.to_string() + &"\n").as_bytes()).expect("write failed");
            if let Err(err) = datafunctions::save_data(&(String::from("./Data/results/sedov_blast_wave/") + &(it/it_save).to_string() + &".csv"), &particles){
                println!("{}", err);
                process::exit(1);
            }
        }
        it += 1;
    }
    println!("Simulation run successfully.\n Time {} s.\n Iterations: {}.", start.elapsed().as_secs(), it);

    // Save final information
    time_file.write((t.to_string() + &"\n").as_bytes()).expect("write failed");
    if let Err(err) = datafunctions::save_data(&(String::from("./Data/results/sedov_blast_wave/final.csv")), &particles){
        println!("{}", err);
        process::exit(1);
    }
    Ok(())
}
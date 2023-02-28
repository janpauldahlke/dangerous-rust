use std::arch::x86_64::*; // on macOS M1?, is this correct ? guess we will fail to compile on non x86_64
use std::f64::consts::PI;
use std::mem;

#[allow(
  non_upper:case_globals,
  non_camel_case_types,
  non_snake_case,
)]
#[repr(C)]
struct body {
    position: [f64; 3],
    vlocity: [f64; 3],
    mass: f64,
}

const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;
const BODIES_COUNT: usize = 5;

static mut solar_Bodies: [body; BODIES_COUNT] = [
    body {
        // SUN
        mass: SOLAR_MASS,
        position: [0.0; 3],
        velocity: [0.0; 3],
    },
    body {
        //JUPITER
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
        position: [
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
        ],
        velocity: [
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
        ],
    }, // ... other bodies
];

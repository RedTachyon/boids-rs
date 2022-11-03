use web_sys::CanvasRenderingContext2d;


const TURN_FACTOR: f32 = 10.0;
const VISUAL_RANGE: f32 = 20.0;
const PROTECTED_RANGE: f32 = 5.0;
const CENTERING_FACTOR: f32 = 0.0005;
const AVOID_FACTOR: f32 = 0.05;
const MATCH_FACTOR: f32 = 0.5;
const MAX_SPEED: f32 = 3.0;
const MIN_SPEED: f32 = 2.0;
const SIZE: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct Boid {
    id: usize,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

impl Boid {
    pub fn render(&self, ctx: &CanvasRenderingContext2d, scale: f64) {
        ctx.begin_path();
        ctx.arc(self.x as f64 * scale, self.y as f64 * scale, 2.0, 0., std::f64::consts::TAU).unwrap();
        ctx.fill();
    }
}

#[derive(Debug, Clone)]
pub struct Simulation {
    boids: Vec<Boid>,
}


impl Simulation {
    pub fn new() -> Self {
        Self {
            boids: vec![],
        }
    }

    pub fn initialize(num: i32) -> Self {
        let mut sim = Self::new();
        for i in 0..num {
            for j in 0..num {
                sim.add_new((i as f32) / 2. + 45., (j as f32) / 2. + 45.);
            }
        }
        sim
    }

    pub fn add_new(&mut self, x: f32, y: f32) {
        let id = self.boids.len();
        self.boids.push(Boid {
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
        });
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d, scale: f64) {
        ctx.clear_rect(0.0, 0.0, 1000., 1000.);
        for boid in &self.boids {
            boid.render(ctx, scale);
        }
    }

    pub fn update(&mut self) {
        let fixed_boids = self.boids.clone();

        for boid in &mut self.boids {
            // Accumulators
            let mut close_dx = 0f32;
            let mut close_dy = 0f32;

            let mut xpos_avg = 0f32;
            let mut ypos_avg = 0f32;

            let mut xvel_avg = 0f32;
            let mut yvel_avg = 0f32;

            let mut neighboring_boids = 0;

            for other_boid in &fixed_boids {
                if boid.id == other_boid.id {
                    continue;
                }

                let dx = boid.x - other_boid.x;
                let dy = boid.y - other_boid.y;

                if (dx.abs() < VISUAL_RANGE) && (dy.abs() < VISUAL_RANGE) {
                    let dist_square = dx * dx + dy * dy;

                    if dist_square < PROTECTED_RANGE * PROTECTED_RANGE {
                        close_dx += dx;
                        close_dy += dy;
                    } else if dist_square < VISUAL_RANGE * VISUAL_RANGE {
                        xpos_avg += other_boid.x;
                        ypos_avg += other_boid.y;

                        xvel_avg += other_boid.vx;
                        yvel_avg += other_boid.vy;

                        neighboring_boids += 1;
                    }
                }
            }

            if neighboring_boids > 0 {

                xpos_avg /= neighboring_boids as f32;
                ypos_avg /= neighboring_boids as f32;

                xvel_avg /= neighboring_boids as f32;
                yvel_avg /= neighboring_boids as f32;

                boid.vx += (xpos_avg - boid.x) * CENTERING_FACTOR
                         + (xvel_avg - boid.vx) * MATCH_FACTOR;

                boid.vy += (ypos_avg - boid.y) * CENTERING_FACTOR
                         + (yvel_avg - boid.vy) * MATCH_FACTOR;

            }

            boid.vx += close_dx * AVOID_FACTOR;
            boid.vy += close_dy * AVOID_FACTOR;

            if boid.x < 0.0 {
                boid.vx += TURN_FACTOR;
            } else if boid.x > SIZE {
                boid.vx -= TURN_FACTOR;
            }

            if boid.y < 0.0 {
                boid.vy += TURN_FACTOR;
            } else if boid.y > SIZE {
                boid.vy -= TURN_FACTOR;
            }

            let speed = (boid.vx * boid.vx + boid.vy * boid.vy).sqrt();

            if speed > MAX_SPEED {
                boid.vx *= MAX_SPEED / speed;
                boid.vy *= MAX_SPEED / speed;
            } else if speed < MIN_SPEED {
                boid.vx *= MIN_SPEED / speed;
                boid.vy *= MIN_SPEED / speed;
            }

            boid.x += boid.vx;
            boid.y += boid.vy;

        }

    }
}
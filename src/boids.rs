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
pub struct Parameters {
    pub turn_factor: f32,
    pub visual_range: f32,
    pub protected_range: f32,
    pub centering_factor: f32,
    pub avoid_factor: f32,
    pub match_factor: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub size_x: f32,
    pub size_y: f32,
}


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
    parameters: Parameters,
}


impl Simulation {
    pub fn new() -> Self {
        let parameters = Parameters {
            turn_factor: TURN_FACTOR,
            visual_range: VISUAL_RANGE,
            protected_range: PROTECTED_RANGE,
            centering_factor: CENTERING_FACTOR,
            avoid_factor: AVOID_FACTOR,
            match_factor: MATCH_FACTOR,
            max_speed: MAX_SPEED,
            min_speed: MIN_SPEED,
            size_x: SIZE,
            size_y: SIZE,
        };
        Self {
            boids: vec![],
            parameters
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

                if (dx.abs() < self.parameters.visual_range) && (dy.abs() < self.parameters.visual_range) {
                    let dist_square = dx * dx + dy * dy;

                    if dist_square <  self.parameters.protected_range * self.parameters.protected_range {
                        close_dx += dx;
                        close_dy += dy;
                    } else if dist_square < self.parameters.visual_range * self.parameters.visual_range {
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

                boid.vx += (xpos_avg - boid.x) * self.parameters.centering_factor
                         + (xvel_avg - boid.vx) * self.parameters.match_factor;

                boid.vy += (ypos_avg - boid.y) * self.parameters.centering_factor
                         + (yvel_avg - boid.vy) * self.parameters.match_factor;

            }

            boid.vx += close_dx * self.parameters.avoid_factor;
            boid.vy += close_dy * self.parameters.avoid_factor;

            if boid.x < 0.0 {
                boid.vx += self.parameters.turn_factor;
            } else if boid.x > self.parameters.size_x {
                boid.vx -= self.parameters.turn_factor;
            }

            if boid.y < 0.0 {
                boid.vy += self.parameters.turn_factor;
            } else if boid.y > self.parameters.size_y {
                boid.vy -= self.parameters.turn_factor;
            }

            let speed = (boid.vx * boid.vx + boid.vy * boid.vy).sqrt();

            if speed > self.parameters.max_speed {
                boid.vx *= self.parameters.max_speed / speed;
                boid.vy *= self.parameters.max_speed / speed;
            } else if speed < self.parameters.min_speed {
                boid.vx *= self.parameters.min_speed / speed;
                boid.vy *= self.parameters.min_speed / speed;
            }

            boid.x += boid.vx;
            boid.y += boid.vy;

        }


    }

    pub fn update_params(&mut self, params: Parameters) {
        self.parameters = params;
    }
}
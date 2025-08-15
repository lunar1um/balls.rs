use ::rand::{prelude::*, rng};
use macroquad::prelude::*;

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    velocity: f32,
    radius: f32,
    color: Color,
}

impl Ball {
    fn attract(&mut self, tx: f32, ty: f32, f: f32, dt: f32) {
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq <= 0.0 {
            return;
        }

        let dist = dist_sq.sqrt();
        let nx = dx / dist;
        let ny = dy / dist;

        let strength = f / dist;
        self.dx += nx * strength * dt;
        self.dy += ny * strength * dt;

        let len = (self.dx * self.dx + self.dy * self.dy).sqrt();
        if len > 0.0 {
            self.dy /= len;
            self.dx /= len;
        }
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }

    fn collides(&self, other: &Ball) -> bool {
        let dx: f32 = self.x - other.x;
        let dy = self.y - other.y;
        let dist_sq = dx * dx + dy * dy;
        let rs = self.radius + other.radius;

        dist_sq <= rs * rs
    }

    fn bounce(&mut self, other: &mut Ball) {
        // normals
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist == 0.0 {
            return;
        }

        let overlap = (self.radius + other.radius) - dist;
        let nx = dx / dist;
        let ny = dy / dist;

        if overlap > 0.0 {
            self.x += nx * overlap / 2.0;
            self.y += ny * overlap / 2.0;
            other.x -= nx * overlap / 2.0;
            other.y -= ny * overlap / 2.0;
        }

        // relative velocity
        let dvx = self.dx - other.dx;
        let dvy = self.dy - other.dy;

        let dot = dvx * nx + dvy * ny;

        if dot > 0.0 {
            return;
        }

        self.dx -= dot * nx;
        self.dy -= dot * ny;
        other.dx += dot * nx;
        other.dy += dot * ny;

        // fatten
        // self.radius += 0.1;
        // other.radius += 0.1;

        // epilepsy
        // self.color = random_color();
        // other.color = random_color();
    }

    fn update_pos(&mut self, dt: f32, speed: &f32) {
        self.x += self.velocity * self.dx * dt * speed;
        self.y += self.velocity * self.dy * dt * speed;
    }

    fn wall_bounce(&mut self) -> usize {
        let mut bounces = 0;

        if self.x + self.radius >= screen_width() {
            self.x = screen_width() - self.radius;
            self.dx *= -1.0;
            bounces += 1;
            // self.color = random_color();
        }

        if self.x - self.radius <= 0.0 {
            self.x = self.radius;
            self.dx *= -1.0;
            bounces += 1;
            // self.color = random_color();
        }

        if self.y + self.radius >= screen_height() {
            self.y = screen_height() - self.radius;
            self.dy *= -1.0;
            bounces += 1;
            // self.color = random_color();
        }

        if self.y - self.radius <= 0.0 {
            self.y = self.radius;
            self.dy *= -1.0;
            bounces += 1;
            // self.color = random_color();
        }

        // self.radius += 0.1 * bounces as f32;

        bounces
    }
}

fn random_color() -> Color {
    let mut rng = rng();

    Color::new(
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0),
        rng.random_range(0.0..1.0),
        1.0,
    )
}

fn generate_balls(
    n: usize,
    min_radius: f32,
    max_radius: f32,
    min_velocity: f32,
    max_velocity: f32,
) -> Vec<Ball> {
    let mut balls = Vec::new();
    let mut rng = rng();

    for _ in 0..n {
        balls.push(Ball {
            x: rng.random_range(20.0..screen_width() - 20.0),
            y: rng.random_range(20.0..screen_height() - 20.0),
            dx: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            dy: if rng.random_bool(0.5) { 1.0 } else { -1.0 },
            velocity: rng.random_range(min_velocity..max_velocity),
            radius: rng.random_range(min_radius..max_radius),
            color: random_color(),
        })
    }

    balls
}

const BASE_TIMESCALE: f32 = 100.0;
const BASE_FORCE: f32 = 1_000.0;

#[macroquad::main("balls")]
async fn main() {
    let mut balls = generate_balls(20, 9.0, 10.0, 20.0, 30.0);
    let mut bounces = 0;
    let mut collisions = 0;
    let mut timescale = BASE_TIMESCALE;
    let mut force = BASE_FORCE;

    loop {
        clear_background(BLACK);

        // keybinds:
        // LEFT or RIGHT -> Pull / Push Force
        // UP or DOWN -> Timescale
        if is_key_down(KeyCode::Down) {
            timescale -= 1.0;
            if timescale < 0.0 {
                timescale = 0.0;
            }
        }

        if is_key_down(KeyCode::Up) {
            timescale += 1.0;
        }

        if is_key_down(KeyCode::Right) {
            force += 50.0;
        }

        if is_key_down(KeyCode::Left) {
            force -= 50.0;
        }

        let m_speed = balls
            .iter()
            .map(|b| b.velocity * timescale)
            .fold(0.0, f32::max);
        let safe_step = balls
            .iter()
            .map(|b| b.radius / m_speed)
            .fold(f32::INFINITY, f32::min)
            .max(1.0 / 1000.0);

        let mut rdt = get_frame_time();

        if is_mouse_button_down(MouseButton::Left) {
            let mouse = mouse_position();

            for ball in &mut balls {
                ball.attract(mouse.0, mouse.1, force, get_frame_time());
            }
        }

        // steps
        while rdt > 0.0 {
            let step_dt = rdt.min(safe_step);

            for ball in &mut balls {
                ball.update_pos(step_dt, &timescale);
                bounces += ball.wall_bounce();
            }

            for i in 0..balls.len() {
                for j in (i + 1)..balls.len() {
                    if balls[i].collides(&balls[j]) {
                        collisions += 1;
                        let (l, r) = balls.split_at_mut(j);
                        l[i].bounce(&mut r[0]);
                    }
                }
            }

            rdt -= step_dt;
        }

        balls.iter_mut().for_each(|ball| ball.draw());

        // info display
        draw_text(
            &format!("Timescale: {}", timescale),
            10.0,
            25.0,
            30.0,
            WHITE,
        );
        draw_text(&format!("Force: {} {}", force, if force < 0.0 { "(PUSH)" } else if force > 0.0 { "(PULL)"} else { "" }), 10.0, 55.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 85.0, 30.0, WHITE);
        draw_text(&format!("Balls: {}", balls.len()), 10.0, 115.0, 30.0, WHITE);
        draw_text(&format!("Bounces: {}", bounces), 10.0, 145.0, 30.0, WHITE);
        draw_text(
            &format!("Collisions: {}", collisions),
            10.0,
            175.0,
            30.0,
            WHITE,
        );

        next_frame().await
    }
}

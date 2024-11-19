use druid::kurbo::Circle;
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, Data, Lens, Point, Vec2, WindowDesc};
use rand::Rng;
use std::sync::Arc;

const GRAVITY: f64 = 0.5;
const DAMPING: f64 = 0.5;
const INTERACTION_RADIUS: f64 = 10.0;
const INTERACTION_FORCE: f64 = 0.05;
const PARTICLE_RADIUS: f64 = INTERACTION_RADIUS / 2.0;

#[derive(Clone, Data, Lens)]
struct AppState {
    particles: Arc<Vec<Particle>>,
}

#[derive(Clone, Data)]
struct Particle {
    position: Point,
    velocity: Vec2,
}

struct SimulationWidget;

impl SimulationWidget {
    fn new() -> Self {
        SimulationWidget
    }
}

impl Widget<AppState> for SimulationWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_anim_frame();
            }
            Event::AnimFrame(_interval) => {
                // Update simulation
                let mut particles = (*data.particles).clone();
                let len = particles.len();

                for i in 0..len {
                    // Use split_at_mut to avoid multiple mutable references
                    let (p1, rest) = particles[i..].split_first_mut().unwrap();
                    // Apply gravity
                    p1.velocity.y += GRAVITY;

                    // Interactions with other particles
                    for p2 in rest.iter_mut() {
                        let dx = p2.position.x - p1.position.x;
                        let dy = p2.position.y - p1.position.y;
                        let dist_sq = dx * dx + dy * dy;
                        if dist_sq < INTERACTION_RADIUS * INTERACTION_RADIUS && dist_sq > 0.0 {
                            let dist = dist_sq.sqrt();
                            let overlap = INTERACTION_RADIUS - dist;
                            let force = overlap * INTERACTION_FORCE;
                            let nx = dx / dist;
                            let ny = dy / dist;
                            let fx = force * nx;
                            let fy = force * ny;
                            p1.velocity.x -= fx;
                            p1.velocity.y -= fy;
                            p2.velocity.x += fx;
                            p2.velocity.y += fy;
                        }
                    }

                    // Update position
                    p1.position += p1.velocity;

                    // Collisions with walls
                    if p1.position.y > ctx.size().height {
                        p1.position.y = ctx.size().height;
                        p1.velocity.y *= -DAMPING;
                    }
                    if p1.position.y < 0.0 {
                        p1.position.y = 0.0;
                        p1.velocity.y *= -DAMPING;
                    }
                    if p1.position.x < 0.0 {
                        p1.position.x = 0.0;
                        p1.velocity.x *= -DAMPING;
                    }
                    if p1.position.x > ctx.size().width {
                        p1.position.x = ctx.size().width;
                        p1.velocity.x *= -DAMPING;
                    }
                }

                data.particles = Arc::new(particles);

                ctx.request_paint();
                ctx.request_anim_frame();
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &AppState,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        // Clear background
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::BLACK);

        // Draw particles
        for particle in data.particles.iter() {
            let circle = Circle::new(particle.position, PARTICLE_RADIUS);
            ctx.fill(circle, &Color::rgba8(0, 0, 255, 128));
        }
    }
}

fn main() {
    let main_window = WindowDesc::new(SimulationWidget::new())
        .window_size((800.0, 600.0))
        .title("2D Water Simulation");

    // Initialize particles
    let mut rng = rand::thread_rng();
    let initial_state = AppState {
        particles: Arc::new(
            (0..500)
                .map(|_| Particle {
                    position: Point::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..600.0)),
                    velocity: Vec2::ZERO,
                })
                .collect(),
        ),
    };

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}

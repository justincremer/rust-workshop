use ggez;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;
use std::env;
use std::path;

struct MainState {
    text: graphics::Text,
    canvas: graphics::Canvas,
    frames: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/iosevka-regular.ttf")?;
        let text = graphics::Text::new(("Hello world!", font, 48.0));
        let canvas = graphics::Canvas::with_window_size(ctx)?;
        let s = MainState {
            text,
            canvas,
            frames: 0,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dest_point = Vec2::new(100.0, 100.0);

        graphics::clear(ctx, Color::WHITE);
        graphics::set_canvas(ctx, Some(&self.canvas));
        graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::new()
                .dest(dest_point)
                .color(Color::BLACK),
        )?;

        graphics::set_canvas(ctx, None);
        graphics::draw(ctx, &self.canvas, graphics::DrawParam::new())?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}\nFRAMES: {}", ggez::timer::fps(ctx), self.frames);
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("hello_world", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

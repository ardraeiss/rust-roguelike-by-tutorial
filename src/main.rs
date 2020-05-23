use rltk::{GameState, Rltk};
use specs_derive::Component;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

// Entity world position
#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}

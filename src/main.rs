use bracket_lib::prelude::*;
enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y, velocity: 0. }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2. {
            self.velocity += 0.2;
        }
        // As i32 always rounds down
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // Velocity makes the player go upwards
        self.velocity = -2.
    }
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            mode: GameMode::Menu,
            frame_time: 0.0,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // Clear screen
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P)lay game");
        ctx.print_centered(9, "(Q)uit game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P)lay Again");
        ctx.print_centered(9, "(Q)uit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Finish later <3
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.;
        self.mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    println!("Hello, world!");
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::new())
}

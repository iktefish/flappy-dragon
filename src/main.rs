use bracket_lib::prelude::*;

/// State machine
enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn constructor(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_mode(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity += -0.2;
    }
}

struct State {
    mode: GameMode,
}

impl State {
    fn constructor() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon!");
        ctx.print_centered(8, "[ P ] to Play game");
        ctx.print_centered(11, "[ Q ] to Quit game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // TODO: Fill this stub leter
        self.mode = GameMode::Menu;

        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "[ P ] Play again");
        ctx.print_centered(11, "[ Q ] Quit game.");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill this stub leter
        self.mode = GameMode::End;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        /*
            ctx.cls();
            ctx.print(1, 1, "Hello, Bracket Terminal here!!!");
        */

        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    /*
        let my_list = ["ONE", "TWO", "THREE"];
        for item in &my_list {
            println!("{}", item);
        }
    */

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(context, State::constructor())
}

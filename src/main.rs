use bracket_lib::prelude::*;

/// State machine
enum GameMode {
    Menu,
    Playing,
    End,
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

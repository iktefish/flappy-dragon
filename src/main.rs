use bracket_lib::prelude::*;

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

    fn main_menu(&mut self, ctx: &mut BTerm) {
        // TODO: Fill this stub leter
        self.mode = GameMode::Playing;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        // TODO: Fill this stub leter
        self.mode = GameMode::Menu;
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

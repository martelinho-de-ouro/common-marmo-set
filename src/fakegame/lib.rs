// `lib.rs` is a crate root so the module tree is:
// crate
//  └── game
//      ├── game_play
//      │   ├── increase_player_score
//      │   └── block_player
//      └── game_rank
//          ├── find_best_player
//          ├── calculate_statistics

fn join_the_game() {}

use game::make_a_fake_telemetry;

use crate::game::game_play::increase_player_score;

mod game {
    pub mod game_play {
        use crate::play_the_game;

        pub fn increase_player_score() {
            crate::player_usage::Player::jump();
        }

        fn block_player() {}
    }

    mod game_rank {
        use super::game_play::increase_player_score;

        fn find_best_player() {}
        fn calculate_statistics() {}
    }

    pub fn make_a_fake_telemetry() {
        // super is similar to `..` related to the path.
        super::join_the_game();
        // in the case of enum, when adding pub all the variants are public.
        print!("{:?}", crate::resources_usage::Resource::Cpu);
    }
}

pub fn play_the_game() {
    // abs path
    crate::game::game_play::increase_player_score();

    // relative path
    game::game_play::increase_player_score();

    // use shortcut usage..
    increase_player_score();

    println!("test");

    make_a_fake_telemetry();
}

mod player_usage {
    // We can restrict the visibility of separated fields when using struct.
    pub struct Player {
        pub nickname: String,
        score: i32,
    }

    impl Player {
        pub fn jump() -> Player {
            Player {
                nickname: String::from("foo"),
                score: 1,
            }
        }
    }
}

mod resources_usage {
    #[derive(Debug)]
    pub enum Resource {
        Cpu,
        Memory,
    }
}
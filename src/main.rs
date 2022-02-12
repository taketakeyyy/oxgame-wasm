mod oxgame;
use rand::Rng;


fn test_battle(debug_print: bool) -> oxgame::JudgeResult {
    let mut grid: Vec<Vec<oxgame::Masu>> = oxgame::make_initialized_grid();

    let mut rng = rand::thread_rng();

    let mut who_turn: i32 = oxgame::WhoTurn::ME;

    while oxgame::judge(&grid).result==oxgame::JudgeResult::CONTINUE {

        if who_turn == oxgame::WhoTurn::ME {
            // 自分は、minimaxを使用
            let node :oxgame::Node = oxgame::run_ai_strategy(&mut grid, who_turn);
            oxgame::put_stone(&mut grid, node.h, node.w, who_turn);
            if debug_print { println!("ME: grid[{}][{}] eval: {}", node.h, node.w, node.eval); }
        }
        else {
            // 相手は、適当に打つ
            loop {
                let rh: usize = rng.gen::<usize>()%3;
                let rw: usize = rng.gen::<usize>()%3;
                if oxgame::is_puttable_stone(&grid, rh, rw) {
                    oxgame::put_stone(&mut grid, rh, rw, who_turn);
                    if debug_print { println!("ENEMY: grid[{}][{}]", rh, rw); }
                    break;
                }
            }
        }
        if debug_print { oxgame::print_grid(&grid); }
        who_turn = who_turn^1;
    }
    // 決着
    let jr = oxgame::judge(&grid);
    if jr.result == oxgame::JudgeResult::ME_WIN {
        println!("ME WIN!");
    }
    else if jr.result == oxgame::JudgeResult::ENEMY_WIN {
        println!("ME LOSE...");
    }
    else {
        println!("DRAW!");
    }
    return jr;
}

fn main() {
    println!("Game Start!");

    const N: i32 = 1000;
    let mut win_num: i32 = 0;
    let mut lose_num: i32 = 0;
    let mut draw_num: i32 = 0;
    for i in 0..N {
        let jr = test_battle(true);
        if jr.result==oxgame::JudgeResult::ME_WIN {
            win_num += 1;
        }
        else if jr.result==oxgame::JudgeResult::ENEMY_WIN {
            lose_num += 1;
        }
        else if jr.result==oxgame::JudgeResult::DRAW {
            draw_num += 1;
        }
        println!("i: {}, win: {}, lose: {}, draw: {}", i, win_num, lose_num, draw_num);
    }
}

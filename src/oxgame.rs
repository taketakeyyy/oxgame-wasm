extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/* 評価値の定数 */
#[wasm_bindgen]
pub struct Eval {}
impl Eval {
    pub const WIN: i32 = 10;
    pub const LOSE: i32 = -10;
    pub const DRAW: i32 = 0;
}

/* 勝敗の構造体 */
#[wasm_bindgen]
pub struct JudgeResult {
    pub result: i32,
}
impl JudgeResult {
    pub const ME_WIN: i32 = 0;
    pub const ENEMY_WIN: i32 = 1;
    pub const DRAW: i32 = 2;
    pub const CONTINUE: i32 = 3;
}

/* マスに誰が書き込んでいるかの構造体 */
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Masu {
    pub masu: i32,
}
impl Masu {
    pub const ME: i32 = 0;
    pub const ENEMY: i32 = 1;
    pub const EMPTY: i32 = -1;
}

/* 誰のターン */
#[wasm_bindgen]
pub struct WhoTurn {}
impl WhoTurn {
    pub const ME: i32 = 0;
    pub const ENEMY: i32 = 1;
}

/* ノード */
#[derive(Default)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Node {
    pub eval: i32,  // 評価値
    pub h: usize,   // h座標
    pub w: usize,   // w座標
}

#[wasm_bindgen]
pub fn make_initialized_grid() -> JsValue {
    let mut grid = vec![];
    grid.push(vec![Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY }]);
    grid.push(vec![Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY }]);
    grid.push(vec![Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY },Masu { masu: Masu::EMPTY }]);
    JsValue::from_serde(&grid).unwrap()
}

pub fn print_grid(grid: &Vec<Vec<Masu>>) {
    println!("+–+-+-+");
    for h in 0..3 {
        print!("|");
        for w in 0..3 {
            if grid[h][w].masu == Masu::ME {
                print!("o");
            }
            else if grid[h][w].masu == Masu::ENEMY {
                print!("x");
            }
            else {
                print!(" ");
            }
            print!("|");
        }
        println!();
        println!("+–+-+-+");
    }
}

/**
 * 勝敗のジャッジ
 */
pub fn judge(grid: &Vec<Vec<Masu>>) -> JudgeResult {
    // 各行を調べる
    for h in 0..3 {
        let target: i32 = grid[h][0].masu;
        if target == Masu::EMPTY { continue; }
        let mut is_win = true;
        for w in 1..3 {
            if grid[h][w].masu != target {
                is_win = false;
            }
        }
        if is_win {
            if target == Masu::ME {
                return JudgeResult{ result: JudgeResult::ME_WIN };
            }
            else {
                return JudgeResult{ result: JudgeResult::ENEMY_WIN };
            }
        }
    }

    // 各列を調べる
    for w in 0..3 {
        let target: i32 = grid[0][w].masu;
        if target == Masu::EMPTY { continue; }
        let mut is_win = true;
        for h in 1..3 {
            if grid[h][w].masu != target {
                is_win = false;
            }
        }
        if is_win {
            if target == Masu::ME {
                return JudgeResult{ result: JudgeResult::ME_WIN };
            }
            else {
                return JudgeResult{ result: JudgeResult::ENEMY_WIN };
            }
        }
    }

    // 斜めを調べる
    {
        let target: i32 = grid[0][0].masu;
        if target != Masu::EMPTY {
            let mut is_win = true;
            for i in 1..3 {
                if grid[i][i].masu != target {
                    is_win = false;
                }
            }
            if is_win {
                if target == Masu::ME {
                    return JudgeResult{ result: JudgeResult::ME_WIN };
                }
                else {
                    return JudgeResult{ result: JudgeResult::ENEMY_WIN };
                }
            }
        }
    }
    {
        let target: i32 = grid[0][2].masu;
        if target != Masu::EMPTY {
            let mut is_win = true;
            for i in 1..3 {
                if grid[0+i][2-i].masu != target {
                    is_win = false;
                }
            }
            if is_win {
                if target == Masu::ME {
                    return JudgeResult{ result: JudgeResult::ME_WIN };
                }
                else {
                    return JudgeResult{ result: JudgeResult::ENEMY_WIN };
                }
            }
        }
    }

    // 全部埋まってたら引き分け
    if is_fill(grid) {
        return JudgeResult { result: JudgeResult::DRAW };
    }

    // 勝敗は決まってないので続行
    return JudgeResult { result: JudgeResult::CONTINUE };
}

/**
 * 全部埋まったか？
 */
pub fn is_fill(grid: &Vec<Vec<Masu>>) -> bool {
    for h in 0..3 {
        for w in 0..3 {
            if grid[h][w].masu == Masu::EMPTY {
                return false;
            }
        }
    }
    return true;
}

fn score(jr: &JudgeResult, depth: i32, h: usize, w: usize) -> Node {
    let node: Node;
    if jr.result == JudgeResult::ME_WIN {
        node = Node {
            eval: Eval::WIN - depth,  // 深さが浅くて勝つほうが評価値が高い（速攻で勝てるほうがいいから）
            h: h,
            w: w,
        };
    }
    else if jr.result == JudgeResult::ENEMY_WIN {
        node = Node {
            eval: Eval::LOSE + depth,  // 深さが深くて負ける方が評価値は高い（粘りたいから）
            h: h,
            w: w,
        };
    }
    else {
        node = Node {
            // eval: Eval::DRAW + depth,  // 引き分けでも、深さが深いほうが評価値が高い（できるだけ引き伸ばしたい）
            eval: Eval::DRAW,
            h: h,
            w: w,
        };
    }
    node
}

pub fn is_puttable_stone(grid: &Vec<Vec<Masu>>, h: usize, w:usize) -> bool {
    grid[h][w].masu == Masu::EMPTY
}

pub fn put_stone(grid: &mut Vec<Vec<Masu>>, h: usize, w: usize, who_turn: i32) {
    if who_turn == WhoTurn::ME {
        grid[h][w].masu = Masu::ME;
    }
    else {
        grid[h][w].masu = Masu::ENEMY;
    }
}

pub fn remove_stone(grid: &mut Vec<Vec<Masu>>, h: usize, w: usize) {
    grid[h][w].masu = Masu::EMPTY;
}

fn minimax(grid: &mut Vec<Vec<Masu>>, who_turn: i32, depth: i32, h: usize, w: usize) -> Node {
    let jr: JudgeResult = judge(grid);

    if jr.result != JudgeResult::CONTINUE {
        return score(&jr, depth, h, w);
    }

    if who_turn == WhoTurn::ME {
        let mut res_node = Node{ eval:i32::MIN, h:99, w:99, };
        for nh in 0..3 {
            for nw in 0..3 {
                if !is_puttable_stone(grid, nh, nw) { continue; }
                put_stone(grid, nh, nw, who_turn);
                let child_node = minimax(grid, who_turn^1, depth+1, nh, nw);
                if child_node.eval > res_node.eval  {
                    res_node = Node{ eval:child_node.eval, h:nh, w:nw};
                }
                remove_stone(grid, nh, nw);
            }
        }
        return res_node;
    }
    else {
        let mut res_node = Node { eval:i32::MAX, h:99, w:99, };
        for nh in 0..3 {
            for nw in 0..3 {
                if !is_puttable_stone(grid, nh, nw) { continue; }
                put_stone(grid, nh, nw, who_turn);
                let child_node = minimax(grid, who_turn^1, depth+1, nh, nw);
                if child_node.eval < res_node.eval {
                    res_node = Node{ eval:child_node.eval, h:nh, w:nw };
                }
                remove_stone(grid, nh, nw);
            }
        }
        return res_node;
    }
}

#[wasm_bindgen]
// pub fn run_ai_strategy(grid: &mut Vec<Vec<Masu>>, who_turn: i32) -> JsValue {
pub fn run_ai_strategy(grid_val: &JsValue, who_turn: i32) -> JsValue {
    let mut grid: Vec<Vec<Masu>> = grid_val.into_serde().unwrap();
    let node: Node = minimax(&mut grid, who_turn, 0, 99, 99);  // 99はダミー
    return JsValue::from_serde(&node).unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn judge_test() {
        let mut grid = make_initialized_grid();
        assert_eq!(judge(&grid).result, JudgeResult::DRAW);

        // 斜めのテスト
        grid = make_initialized_grid();
        grid[0][0].masu = Masu::ME;
        grid[1][1].masu = Masu::ME;
        grid[2][2].masu = Masu::ME;
        assert_eq!(judge(&grid).result, JudgeResult::ME_WIN);

        // 斜めのテスト
        grid = make_initialized_grid();
        grid[0][2].masu = Masu::ENEMY;
        grid[1][1].masu = Masu::ENEMY;
        grid[2][0].masu = Masu::ENEMY;
        assert_eq!(judge(&grid).result, JudgeResult::ENEMY_WIN);

        // 横のテスト
        grid = make_initialized_grid();
        grid[0][0].masu = Masu::ME;
        grid[0][1].masu = Masu::ENEMY;
        grid[0][2].masu = Masu::ME;
        assert_eq!(judge(&grid).result, JudgeResult::DRAW);

        // 不定形
        grid = make_initialized_grid();
        grid[0][0].masu = Masu::EMPTY;
        grid[0][1].masu = Masu::EMPTY;
        grid[0][2].masu = Masu::EMPTY;
        grid[1][0].masu = Masu::ME;
        grid[1][1].masu = Masu::ENEMY;
        grid[1][2].masu = Masu::EMPTY;
        grid[2][0].masu = Masu::EMPTY;
        grid[2][1].masu = Masu::EMPTY;
        grid[2][2].masu = Masu::ME;
        assert_eq!(judge(&grid).result, JudgeResult::DRAW);

    }
}
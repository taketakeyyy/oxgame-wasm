import * as oxgame from "../pkg/oxgame";

// wasm.greet();

// {
//     let box = [0, 11, 222];
//     wasm.test_box1(box);
// }

// {
//     let node = new wasm.Node(1,2,3);
//     let box = [1,2,3];
//     wasm.test_box1(box);
// }

{
    console.log("START")
    let grid = oxgame.make_initialized_grid();
    console.log(grid);
    let node_val = oxgame.run_ai_strategy(grid, 0);
    console.log(node_val);
}
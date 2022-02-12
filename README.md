# oxgame-wasm

This is a WASM version of [oxgame(tic-tac-toe)](https://github.com/taketakeyyy/oxgame) program.


# npm install

https://www.npmjs.com/package/@taketakeyyy/oxgame

```sh
> npm i @taketakeyyy/oxgame
```

# Usage

## import
```
import * as oxgame from "../pkg/oxgame";
```

## make_initialized_grid
Makes initialized grid.

```javascript
let grid = oxgame.make_initialized_grid();
// "masu": -1 (Empty)
// "masu": 0 (player0)
// "masu": 1 (player1)
//
// [
//     [
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         }
//     ],
//     [
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         }
//     ],
//     [
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         },
//         {
//             "masu": -1
//         }
//     ]
// ]
```

## run_ai_strategy
Returns optimal placement.

```javascript
let node = oxgame.run_ai_strategy(grid, 0);
// {
//     "eval": 0,
//     "h": 0,
//     "w": 0
// }
```
* eval: evaluation value
* player0's optimal placement is ...
  - `grid[node.h][node.w].masu = 0;`

# For development
See: [README_dev.md](README_dev.md)
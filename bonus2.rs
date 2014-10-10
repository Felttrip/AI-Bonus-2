//libs
use std::collections;
use std::vec;
use std::rand;
use std::io;
use std::num;

static size: int = 3;


struct gameState{
	 board : [[uint, ..3], ..3],
	 moves_made : uint,
	 zero_pos : (int, int)
}

fn main() {

	let mut game : gameState;
	game.zero_pos = (2i,2i);
	game = generatePuzzle();
	for x in range(0i, 3i){
		for y in range(0i, 3i){
			println!("{}",game.board[x as uint][y as uint]);
		}
	}
}

fn generatePuzzle() -> gameState{
	//new game state
	let mut puzzle = gameState{
		board: [[1,2,3], [4,5,6], [7,8,0]],
		zero_pos: (2i, 2i),
		moves_made: 0u
	};


	//scramble the board
	//find random number of moves to make
	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	//valid moves
	let mut validMoves : Vec<(uint, uint)>;

	//tile that will be selected
	let mut tileToMove :(uint,uint) = (0,0);

	for x in range(0i, num_rand_moves as int){
		validMoves = moveableTiles(puzzle.zero_pos);
		tileToMove = validMoves[rand::random::<uint>()%validMoves.len()];
		//move tile
		//assign the value of the of the tile to move to the location of the 0 tile
		puzzle.board[(puzzle.zero_pos.val0()) as uint][(puzzle.zero_pos.val1()) as uint] = puzzle.board[tileToMove.val0()][tileToMove.val1()];
		//move the zero tile to the old tile's location
		puzzle.board[tileToMove.val0()][tileToMove.val1()] = 0u;
		puzzle.zero_pos = (tileToMove.val0() as int, tileToMove.val1() as int);
		validMoves = Vec::new();
	}

	return puzzle;
}

/* If there are no valid moves, -1 is returned */
fn moveableTiles( zero_pos : (int, int) ) -> Vec<(uint, uint)>{

	let mut valid_moves : Vec<(uint, uint)> = Vec::new();

	/* Can move right? */
	if( zero_pos.val0() + 1 <= size - 1  ){
		valid_moves.push(((zero_pos.val0()+1) as uint,(zero_pos.val1()) as uint));
	}
	/* Can move left? */
	if( zero_pos.val0() - 1 >= 0 ){
		valid_moves.push(((zero_pos.val0()-1) as uint,(zero_pos.val1()) as uint));
	}
	/* Can move up? */
	if( zero_pos.val1() + 1 <= size - 1 ){
		valid_moves.push(((zero_pos.val0()) as uint,(zero_pos.val1()+1) as uint));
	}
	/* Can move down? */
	if( zero_pos.val1() - 1 >= 0 ){
		valid_moves.push(((zero_pos.val0()) as uint,(zero_pos.val1()-1) as uint));
	}

	return valid_moves;
}


// fn Astar( puzzle : gameState ){
//
// }
//
// fn manhattenDistance( puzzle : gameState ) -> uint{
//
// 	let mut i : uint = 0;
// 	let mut j : uint = 0;
// 	let mut dist : uint = 0;
//
// 	/* For every tile */
// 	for i in (0u, size){
// 		for j in (0u, size){
// 			let cur_value = puzzle.board[i][j];
// 			if(cur_value == 0){
// 				continue;
// 			}
// 			let final_pos : (uint, uint) = (cur_value/size, cur_value % size);
// 			dist = dist + abs(final_pos.val0() - i) + abs(final_pos.val1() - j);
// 			println!("{}", dist);
// 		}
// 	}
//
// 	return dist;
// }

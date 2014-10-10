//libs
use std::collections;
use std::vec;
use std::rand;
use std::io;
use std::num;

static size: int = 3;


struct gameState{
	board : Vec<Vec<uint>>,
	moves_made : uint,
	zero_pos : (int, int)
}

fn main() {

	let mut game : gameState;
	game.zero_pos = (2i,2i);
	game = generatePuzzle();

}

fn generatePuzzle() -> gameState{
	//new game state
	let mut puzzle = gameState{
		board: vec![vec![1,2,3], vec![4,5,6], vec![7,8,0]],
		zero_pos: (1i, 1i),
		moves_made: 0u
	};


	//find random number of moves to make
	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	//scramble the board
	//for x in range(0i, 10i){
		//get valid moves
		println!("Valid moves = {}",moveableTiles(puzzle.zero_pos));
//	}

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

//libs
use std::collections;
use std::vec;
use std::rand;
use std::io;

static size: uint = 3;


struct gameState{
	board : Vec<Vec<uint>>,
	moves_made : uint,
	zero_pos : (uint, uint)
}

fn main() {
	
	let mut game : gameState;
	game.zero_pos = (2u,2u);
	game = generatePuzzle(game);

}

fn generatePuzzle(game : gameState) -> gameState{

	let mut board : Vec<Vec<uint>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,0]];
	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	validMove(game.zero_pos);

	return board;
}

/* If there are no valid moves, -1 is returned */
fn validMove( zero_pos : (uint, uint) ) -> Vec<(uint, uint)>{

	let mut valid_moves : Vec<(uint, uint)>;

	/* Can move right? */
	if( zero_pos.val0() + 1 <= size - 1 ){
		valid_moves.push((zero_pos.val0()+1,zero_pos.val1()));
	}
	/* Can move left? */
	if( zero_pos.val0() - 1 >= 0 ){
		valid_moves.push((zero_pos.val0()+1,zero_pos.val1()));
	}
	/* Can move up? */
	if( zero_pos.val1() + 1 <= size - 1 ){
		valid_moves.push((zero_pos.val0()+1,zero_pos.val1()));
	}
	/* Can move down? */
	if( zero_pos.val1() - 1 >= 0 ){
		valid_moves.push((zero_pos.val0()+1,zero_pos.val1()));
	}

	return valid_moves;
}

//libs
use std::collections;
use std::vec;
use std::rand;
use std::io;
use std::num;


static size: uint = 3;


struct gameState{
	board : Vec<Vec<uint>>,
	moves_made : uint,
	zero_pos : (uint, uint)
}

fn main() {
	
	let mut game : gameState = generatePuzzle();

}

fn generatePuzzle() -> gameState{


	let mut board : Vec<Vec<uint>> = vec![vec![1,2,3], vec![4,5,6], vec![7,8,0]];
	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	//validMove(game.zero_pos);
	manhattenDistance


	return game;
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

fn Astar( puzzle : gameState ){

}

fn manhattenDistance( puzzle : gameState ) -> uint{

	let mut i : uint = 0;
	let mut j : uint = 0;
	let mut dist : uint = 0;

	/* For every tile */
	for i in (0u, size){
		for j in (0u, size){
			let cur_value = puzzle.board[i][j];
			if(cur_value] == 0){
				continue;
			}
			let final_pos : (uint, uint) = (cur_value/size, cur_value % size);
			dist = dist + abs(final_pos.val0() - i) + abs(final_pos.val1() - j);
			println!("{}", dist);
		}
	}

	return dist;
}




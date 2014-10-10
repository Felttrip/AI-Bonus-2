//libs
use std::collections::PriorityQueue;
use std::vec;
use std::rand;
use std::io;
use std::num;


static size: int = 3;

struct gameState{
	board : [[uint, ..3], ..3],
	moves_made : uint,
	zero_pos : (int, int),
	manhattenDistance : uint
}

impl Eq for gameState{

}

impl PartialEq for gameState{
	fn eq(&self, other: &gameState) -> bool{
		return self.manhattenDistance == other.manhattenDistance; 
	}

	fn ne(&self, other: &gameState) -> bool{
		return self.manhattenDistance != other.manhattenDistance;
	}
}

impl Ord for gameState {
	fn cmp(&self, other: &gameState) -> Ordering{
		self.manhattenDistance.cmp(&other.manhattenDistance)
	}
}

impl PartialOrd for gameState {
    fn partial_cmp(&self, other: &gameState) -> Option<Ordering>{
    	Some(self.cmp(other))
    }
}

impl Clone for gameState {
	fn clone(&self) -> gameState{
		gameState { board: self.board, moves_made: self.moves_made, zero_pos : self.zero_pos, manhattenDistance : self.manhattenDistance }
	}
}

fn main() {

	let mut game : gameState;
	game.zero_pos = (2i,2i);
	game = generatePuzzle();

}

fn generatePuzzle() -> gameState{
	//new game state
	let mut puzzle = gameState{
		board: vec![vec![4,1,3], vec![0,2,5], vec![7,8,6]],
		zero_pos: (1i, 1i),
		moves_made: 0u,
		manhattenDistance: 0u
	};


	//find random number of moves to make
	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	//scramble the board
	//for x in range(0i, 10i){
		//get valid moves
		println!("Valid moves = {}",moveableTiles(puzzle.zero_pos));
//	}

	let mut dist : int = manhattenDistance(&puzzle);
	println!("{}", dist)

	Astar(&puzzle);

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


fn Astar( initialState : & gameState ) -> int{
	
	/* Path cost */
	let mut path_cost : int;
	let mut pq : PriorityQueue<gameState> = PriorityQueue::new();

	return 1;

}

/* Return an inverse since the PQ is max only */
fn manhattenDistance( puzzle  : & gameState  ) -> int{

	let mut i : int = 0;
	let mut j : int = 0;
	let mut dist :  int = 0;

	/* For every tile */
	for i in range(0i, (size)){
		for j in range(0i, (size)){
			let cur_value = puzzle.board[i as uint][j as uint];
			if(cur_value == 0){
				continue;
			}
			let final_pos : (int, int) = (((cur_value -1) /(size) as uint) as int, (((cur_value - 1) % (size) as uint)) as int);
			dist = dist + num::abs::<int>(final_pos.val0() - i) + num::abs::<int>(final_pos.val1() - j);
			println!("dist = {}, x val {}, y val {}, cur_val {}", dist, num::abs::<int>(final_pos.val0() - i), num::abs::<int>(final_pos.val1() - j), cur_value);
		}
	}

	return dist * -1;
}

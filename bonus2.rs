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
	inverseManhattenDistance : int
}
 
impl Eq for gameState{

}

impl PartialEq for gameState{
	fn eq(&self, other: &gameState) -> bool{
		return self.inverseManhattenDistance == other.inverseManhattenDistance; 
	}

	fn ne(&self, other: &gameState) -> bool{
		return self.inverseManhattenDistance != other.inverseManhattenDistance;
	}
}

impl Ord for gameState {
	fn cmp(&self, other: &gameState) -> Ordering{
		self.inverseManhattenDistance.cmp(&other.inverseManhattenDistance)
	}
}

impl PartialOrd for gameState {
    fn partial_cmp(&self, other: &gameState) -> Option<Ordering>{
    	Some(self.cmp(other))
    }
}

impl Clone for gameState {
	fn clone(&self) -> gameState{
		gameState { board: self.board, moves_made: self.moves_made, zero_pos : self.zero_pos, inverseManhattenDistance : self.inverseManhattenDistance }
	}
 }
 
 fn main() {
 	let mut game : gameState;
 	game.zero_pos = (2i,2i);
 	game = generatePuzzle();

 	manhattenDistance(&game);
 	println!("{}", game.inverseManhattenDistance);
 }
 
 fn generatePuzzle() -> gameState{
 	//new game state
 	let mut puzzle = gameState{
		board: [[1,2,3], [4,5,6], [7,8,0]],
		zero_pos: (2i, 2i),
		moves_made: 0u,
		inverseManhattenDistance: 0i
 	};
 
 
	//scramble the board
 	//find random number of moves to make
 	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;
 
	//valid moves
	let mut validMoves : Vec<(uint, uint)>;
	//scramble the board
 
	//tile that will be selected
	let mut tileToMove :(uint,uint) = (0,0);
	manhattenDistance(&puzzle);

 
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


	Astar(&puzzle);
 
 	return puzzle;
 }
 
fn Astar( initialState : & gameState ) -> int{
	
	/* Path cost */
	let mut path_cost : int;
	let mut pq : PriorityQueue<gameState> = PriorityQueue::new();

	return 1;

}
/* Return an inverse since the PQ is max only */
fn manhattenDistance( puzzle  : & gameState  ){

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

	//puzzle.inverseManhattenDistance = dist * -1;

	println!("{}", dist * -1);

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

//libs
use std::collections::PriorityQueue;
use std::collections::TrieSet;
use std::vec;
use std::rand;
use std::io;
use std::num;

static size: int = 3;

struct gameState{
	board : [[uint, ..3], ..3],
	moves_made : uint,
	zero_pos : (int, int),
	inverseManhattenDistance : int,
	state_id : uint
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
		gameState { board: self.board, moves_made: self.moves_made, zero_pos : self.zero_pos, inverseManhattenDistance : self.inverseManhattenDistance, state_id : self.state_id }
	}
 }

 fn main() {
 	let mut game : gameState;
 	game = generatePuzzle();
 	let mut moves_needed = Astar(game);
 	println!("{}", moves_needed);


 }

 fn generatePuzzle() -> gameState{
 	//new game state
 	let mut puzzle = gameState{
		board: [[1,2,3], [4,5,6], [7,8,0]],
		zero_pos: (2i, 2i),
		moves_made: 0u,
		inverseManhattenDistance: 0i,
		state_id: 123456780u
 	};

	//scramble the board
 	//find random number of moves to make
 	let mut num_rand_moves = (rand::random::<uint>() % 500u) + 1u;

	//valid moves
	let mut validMoves : Vec<(uint, uint)>;
	//scramble the board

	//tile that will be selected
	let mut tileToMove :(uint,uint) = (0,0);
	manhattenDistance(puzzle);


	for x in range(0i, num_rand_moves as int){
		validMoves = moveableTiles(puzzle.zero_pos);
		tileToMove = validMoves[rand::random::<uint>()%validMoves.len()];
		//move tile
		//assign the value of the of the tile to move to the location of the 0 tile
		puzzle.board[(puzzle.zero_pos.val0()) as uint][(puzzle.zero_pos.val1()) as uint] = puzzle.board[tileToMove.val0()][tileToMove.val1()];
		//move the zero tile to the old tile's location
		puzzle.board[tileToMove.val0()][tileToMove.val1()] = 0u;
		puzzle.zero_pos = (tileToMove.val0() as int, tileToMove.val1() as int);
		puzzle.state_id = generateStateId(puzzle);
		validMoves = Vec::new();
	}

 	return puzzle;
 }

/*
 *	A star returns the number of moves needed to solve the problem
 *  -1 indicates that no solution exists.
 */

fn Astar( initialState : gameState ) -> int{

	/* Path cost */
	let mut path_cost : int;
	let mut pq : PriorityQueue<gameState> = PriorityQueue::new();
    let mut explored = TrieSet::new();
	pq.push(initialState);

	loop{
		if(pq.len() == 0){
			return -1;
		}
		let mut next = pq.pop();
		let active_state = next.unwrap();
		if( isGoal(active_state) ){
			return active_state.moves_made as int;
		}
		explored.insert(active_state.state_id);
		let mut next_states : Vec<gameState> =  generateSuccessorStates(active_state);
		for x in range(0u, next_states.len()){
			if(explored.contains(&next_states[x].state_id)){
				continue;
			}

			/*had to do this */
			let mut puzzle = gameState{
				board: next_states[x].board,
				zero_pos: next_states[x].zero_pos,
				moves_made: next_states[x].moves_made,
				inverseManhattenDistance: next_states[x].inverseManhattenDistance - next_states[x].moves_made as int,
				state_id: next_states[x].state_id
 			};

 			/*wanted to do this, and to push next_states[x] */
			//next_states[x].inverseManhattenDistance = next_states[x].inverseManhattenDistance - next_states[x].moves_made as int;
			pq.push(puzzle);
		}
	}
}

fn generateStateId(state : gameState) -> uint{

	let mut id : uint = 0;

	for x in range(0u, 3u){
		for y in range(0u, 3u){
			id += state.board[x][y] * num::pow::<uint>(10u,  (x * 3) + y % 3);
		}
	}
	return id;
}

fn printBoard( board: gameState ){

	for x in range(0u, 3u){
		for y in range(0u, 3u){
			print!("{}", board.board[x][y]);
		}
		print!("\n");
	}
	print!("\n");

}

fn isGoal( testState: gameState ) -> bool{

	for x in range(0u, 3u){
		for y in range(0u,3u){
			let mut correct_tile = (x*3) + 1 + (y%3);
			if(x==2&&y==2){
				correct_tile = 0;
			}
			if(testState.board[x][y] != correct_tile ){
				println!("not a goal");
				return false;
			}
		}
	}
	println!("is a goal");
	return true;
}

/* Return an inverse since the PQ is max only */
fn manhattenDistance( puzzle  : gameState  ) -> int{

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
			//println!("dist = {}, x val {}, y val {}, cur_val {}", dist, num::abs::<int>(final_pos.val0() - i), num::abs::<int>(final_pos.val1() - j), cur_value);
		}
	}

	//puzzle.inverseManhattenDistance = dist * -1;

	//println!("{}", dist * -1);
	return dist;

}

fn generateSuccessorStates( parentState : gameState ) ->  Vec<gameState>{

	let valid_moves : Vec<(uint, uint)> = moveableTiles(parentState.zero_pos);
	let mut successor_states : Vec<gameState> = Vec::new();

	for x in range(0u, valid_moves.len()){
		successor_states.push(makeMove( parentState, (valid_moves[x].val0() as int, valid_moves[x].val1() as int)));
	}

	for x in range(0u, successor_states.len()){
		println!("{}", successor_states[x].inverseManhattenDistance);
	}

	return successor_states;
}

fn makeMove( init_state : gameState,  new_zero : (int, int) ) -> gameState{

	let mut new_state = gameState{
		board : init_state.board,
		zero_pos : new_zero,
		moves_made : 1 + init_state.moves_made,
		inverseManhattenDistance : 0,
		state_id : 0,
	};

	let previous_value : uint = new_state.board[new_zero.val0() as uint][new_zero.val1() as uint];
	new_state.board[new_zero.val0() as uint][new_zero.val1() as uint] = 0;
	new_state.board[init_state.zero_pos.val0() as uint][init_state.zero_pos.val1() as uint] = previous_value;
	new_state.inverseManhattenDistance = manhattenDistance(new_state);
	new_state.state_id = generateStateId(new_state);

	return new_state;
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

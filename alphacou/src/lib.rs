use ascacou::{Board, Color, Move};
use indextree::{Arena, NodeId};
use std::time::{Duration, Instant};

#[derive(Debug)]
struct NodeData {
	/// Number of times a node or its children has been visited.
	visits: u32,
	/// Computed UCB based on other values, it is stored as an
	/// i64 to avoid any floating point issue.
	uct: i64,
	/// Sum of simulation scores.
	score: i32,
	// The root node has no move associated.
	mov: Move,
}

impl NodeData {
	fn new(mov: Move) -> NodeData {
		NodeData {
			mov,
			visits: 0,
			score: 0,
			uct: 0,
		}
	}
}

pub struct Solver {
	arena: Arena<NodeData>, // WARN: our usage of this arena is not thread safe, beware.
	root: NodeId,
	iterations: usize,
}

impl Solver {
	pub fn best_move(board: Board, expected_time: Duration) -> Option<Move> {
		Solver::solve(board, expected_time).current_best_move()
	}

	pub fn best_continuation(board: Board, expected_time: Duration) -> Vec<Move> {
		Solver::solve(board, expected_time).current_best_continuation()
	}

	fn solve(board: Board, expected_time: Duration) -> Solver {
		let start = Instant::now();
		let mut solver = Solver::new();
		while expected_time > (Instant::now() - start) {
			solver.run_search_iteration(board)
		}
		eprintln!(" max depth: {}", solver.tree_depth(solver.root));
		eprintln!("iterations: {}", solver.iterations);
		eprintln!(
			"moves: {}",
			solver
				.root
				.children(&solver.arena)
				.map(|id| format!("{}", solver.get_data(id).visits))
				.collect::<Vec<_>>()
				.join(", ")
		);
		solver
	}

	fn new() -> Solver {
		let mut arena = Arena::new();
		let dumb_mov = Move::new(0, 0, Color::White);
		Solver {
			root: arena.new_node(NodeData::new(dumb_mov)),
			arena,
			iterations: 0,
		}
	}

	fn current_best_move(&self) -> Option<Move> {
		self.best_next_node(self.root)
			.map(|node_id| self.get_data(node_id).mov)
	}

	fn current_best_continuation(&self) -> Vec<Move> {
		let mut vec = Vec::with_capacity(25);
		let mut node_id = self.root;
		while let Some(child) = self.best_next_node(node_id) {
			node_id = child;
			vec.push(self.get_data(node_id).mov);
		}
		vec
	}

	fn best_next_node(&self, node_id: NodeId) -> Option<NodeId> {
		node_id
			.children(&self.arena)
			.max_by_key(|id| self.arena.get(*id).map(|node| node.get().visits))
	}

	fn run_search_iteration(&mut self, board: Board) {
		self.iterations += 1;
		let (model, node_id) = self.select(board);
		let (model, new_node_id) = self.expand(model, node_id);
		let val = Solver::simulate(model);
		self.backpropagate(new_node_id, val);
	}

	fn select(&self, board: Board) -> (Board, NodeId) {
		let mut board = board;
		let mut id = self.root;
		while self.fully_explored(id, board) {
			match self.best_child_uct(id) {
				Some(node_id) => id = node_id,
				None => break,
			}
			board = board.next(self.get_data(id).mov);
		}

		(board, id)
	}

	fn expand(&mut self, board: Board, node_id: NodeId) -> (Board, NodeId) {
		use rand::seq::IteratorRandom;
		let mut rng = rand::rng();

		let already_expanded_moves: Vec<Move> = node_id
			.children(&self.arena)
			.map(|id| self.get_data(id).mov)
			.collect();
		if let Some(mov) = board
			.possible_moves()
			.into_iter()
			.filter(|mov| !already_expanded_moves.contains(mov))
			.choose(&mut rng)
		{
			let new_node_id = self.arena.new_node(NodeData::new(mov));
			node_id.append(new_node_id, &mut self.arena);
			(board.next(mov), new_node_id)
		} else {
			(board, node_id)
		}
	}

	fn simulate(board: Board) -> i32 {
		use rand::seq::IteratorRandom;

		let mut board = board;
		let mut current_player = 1i8;
		let mut rng = rand::rng();

		while let Some(mov) = board.possible_moves().into_iter().choose(&mut rng) {
			current_player = -current_player;
			board = board.next(mov);
		}
		(board.current_score() * current_player).clamp(-1, 1) as i32
	}

	fn backpropagate(&mut self, node_id: NodeId, value: i32) {
		let mut current_player = 1;
		for id in node_id.ancestors(&mut self.arena).collect::<Vec<NodeId>>() {
			let parent_visits = if let Some(pid) = self.arena.get(id).unwrap().parent() {
				// We add 1 there because the visit count has not been updated yet.
				self.arena.get(pid).map(|node| node.get()).unwrap().visits + 1
			} else {
				0
			};
			let data = self.arena.get_mut(id).map(|node| node.get_mut()).unwrap();
			data.visits += 1;
			data.score += value * current_player;
			data.uct = Solver::compute_uct(&data, parent_visits);
			current_player = -current_player;
		}
	}

	fn best_child_uct(&self, node_id: NodeId) -> Option<NodeId> {
		node_id
			.children(&self.arena)
			.max_by(|a, b| self.get_data(*a).uct.cmp(&self.get_data(*b).uct))
	}

	fn fully_explored(&self, node: NodeId, board: Board) -> bool {
		node.children(&self.arena).count() == board.possible_moves().len()
	}

	fn get_data(&self, node_id: NodeId) -> &NodeData {
		self.arena.get(node_id).map(|node| node.get()).unwrap()
	}

	fn tree_depth(&self, root: NodeId) -> usize {
		root.children(&self.arena)
			.map(|nid| self.tree_depth(nid) + 1)
			.max()
			.unwrap_or(0)
	}

	fn compute_uct(data: &NodeData, parent_visits: u32) -> i64 {
		let int_factor = 100_000_000f64;
		let expoitation = data.score as f64 / data.visits as f64;
		let exploration = (2f64 * (parent_visits as f64).log2() / data.visits as f64).sqrt();

		(int_factor * (expoitation + exploration)) as i64
	}
}

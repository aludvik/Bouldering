extends Node

var levels: Array = []
var level: int = 0

func _ready():
	levels = [
		[
			$Board.Piece.Hole, null, null, null,
			null, null, null, null,
			null, null, $Board.Piece.Boulder, null,
			null, null, null, $Board.Piece.Tractor
		],
		[
			$Board.Piece.Block, $Board.Piece.Block, null, null,
			null, null, $Board.Piece.Hole, null,
			$Board.Piece.Tractor, $Board.Piece.Boulder, $Board.Piece.Block, null,
			null, null, null, null
		],
		[
			null, $Board.Piece.Hole, null, null,
			null, $Board.Piece.Block, null, null,
			null, $Board.Piece.Boulder, $Board.Piece.Tractor, null,
			null, null, null, null
		],
		[
			$Board.Piece.Hole, $Board.Piece.Block, null, null,
			null, $Board.Piece.Boulder, $Board.Piece.Boulder, null,
			null, null, null, null,
			$Board.Piece.Tractor, null, $Board.Piece.Block, $Board.Piece.Hole
		],
		[
			null, null, $Board.Piece.Hole, null,
			$Board.Piece.Hole, $Board.Piece.Tractor, $Board.Piece.Boulder, $Board.Piece.Block,
			null, null, $Board.Piece.Boulder, null,
			$Board.Piece.Block, null, null, null
		]
	]
	set_initial_board_state()
	$Board.init_board()

func set_initial_board_state():
	$Board.initial_state = levels[level]

func _input(event):
	if event is InputEventKey:
		$Board.reset_board()

func _on_Board_game_finished():
	level += 1
	if level == levels.size():
		get_tree().notification(MainLoop.NOTIFICATION_WM_QUIT_REQUEST)
		return
	set_initial_board_state()
	$Board.reset_board()

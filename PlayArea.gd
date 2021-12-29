extends Node2D

signal game_finished

export var size: int = 0

func _ready():
	var s = Vector2(size, size)
	$Board.max_grid_size = s
	$ColorRect.set_size(s)

func _on_Board_game_finished():
	emit_signal("game_finished")

func init_board(initial_state) -> bool:
	var board_size = guess_board_size(initial_state.size())
	if board_size == 0:
		print("Couldn't determine board size: %d" % initial_state)
		return false
	$Board.initial_state = initial_state
	$Board.board_size = board_size
	$Board.init_board()
	center_board()
	return true

func center_board():
	var margin = $Board.max_grid_size - $Board.grid_size
	$Board.position = margin / 2

func guess_board_size(n):
	for i in range(3, n/2):
		if i * i == n:
			return i
	return 0

extends Node2D

signal game_finished

export var size: int = 0

func _ready():
	var s = Vector2(size, size)
	$Board.max_grid_size = s
	$ColorRect.set_size(s)

func _on_Board_game_finished():
	emit_signal("game_finished")

func init_board(level) -> bool:
	$Board.initial_state = level.cells
	$Board.board_size = level.size
	$Board.init_board()
	center_board()
	return true

func center_board():
	var margin = $Board.max_grid_size - $Board.grid_size
	$Board.position = margin / 2

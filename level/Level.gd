extends Node

var level

var Piece = preload("res://level/Piece.gd")

func _ready():
	set_initial_board_state()

func set_initial_board_state():
	return $PlayArea.init_board(level)

func _input(event):
	if event is InputEventKey:
		set_initial_board_state()

func _on_Board_game_finished():
	var LevelSelect = load("res://level_select/LevelSelect.tscn")
	var level_select = LevelSelect.instance()
	var root = get_tree().get_root()
	root.remove_child(self)
	self.queue_free()
	root.add_child(level_select)

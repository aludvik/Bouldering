extends Node

var level
var index

var Piece = preload("res://level/Piece.gd")

func _ready():
	set_initial_board_state()

func set_initial_board_state():
	return $PlayArea.init_board(level)

func _on_Board_game_finished():
	State.completion[index] = true
	return_to_level_select()

func _on_Back_pressed():
	return_to_level_select()
	
func return_to_level_select():
	var LevelSelect = load("res://level_select/LevelSelect.tscn")
	var level_select = LevelSelect.instance()
	var root = get_tree().get_root()
	root.remove_child(self)
	self.queue_free()
	root.add_child(level_select)

func _on_Reset_pressed():
	set_initial_board_state()

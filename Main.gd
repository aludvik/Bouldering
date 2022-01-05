extends Node

export var start_level = ""
export var level_dir = "res://levels/pub"

var levels: Array = []
var level: int = 0

var Piece = preload("Piece.gd")

func _ready():
	load_levels()
	if levels.size() == 0:
		quit()
		return
	skip_levels()
	if !set_initial_board_state():
		quit()

func skip_levels():
	if start_level != "":
		var level_idx = 0
		for lvl in levels:
			if start_level == lvl.name:
				level = level_idx
				break
			level_idx += 1

func set_initial_board_state():
	return $PlayArea.init_board(levels[level])

func _input(event):
	if event is InputEventKey:
		set_initial_board_state()

func quit():
	get_tree().notification(MainLoop.NOTIFICATION_WM_QUIT_REQUEST)

func _on_Board_game_finished():
	level += 1
	if level == levels.size():
		quit()
		return
	set_initial_board_state()

func load_levels():
	levels = preload("res://LevelList.gd").new().levels

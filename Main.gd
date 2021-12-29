extends Node

export var start_level = ""
export var level_dir = "res://levels/pub"

var levels: Array = []
var level_names: Array = []
var level: int = 0

func _ready():
	var loaded_levels = load_levels()
	if levels.size() == 0:
		quit()
		return
	skip_levels()
	if !set_initial_board_state():
		quit()

func skip_levels():
	if start_level != "":
		var level_idx = 0
		for n in level_names:
			if start_level == n:
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

func read_file(path):
	var file = File.new()
	if !file.file_exists(path):
		print("File not found: %s" % path)
	file.open(path, File.READ)
	var content = file.get_as_text()
	file.close()
	return content

func read_level(name):
	var path = "%s/%s" % [level_dir, name]
	var data = read_file(path)
	var level = []
	for c in data:
		match c:
			"+", "-", "|", "\n":
				pass
			" ":
				level.append(null)
			"O":
				level.append($PlayArea/Board.Piece.Hole)
			"*":
				level.append($PlayArea/Board.Piece.Boulder)
			"t":
				level.append($PlayArea/Board.Piece.Tractor)
			"#":
				level.append($PlayArea/Board.Piece.Block)
			_:
				print("Invalid level: %s" %  name)
				return null
	return level

func load_levels():
	var dir = Directory.new()
	if dir.open(level_dir) == OK:
		dir.list_dir_begin(true, true)
		var level_name = dir.get_next()
		while level_name != "":
			if dir.current_is_dir():
				print("Found directory in res://levels")
				return []
			level_names.append(level_name)
			level_name = dir.get_next()
		dir.list_dir_end()
	else:
		print("An error occurred when trying to access the path.")
		return []
	level_names.sort()
	var i = 0
	while i < level_names.size():
		var level_name = level_names[i]
		var level = read_level(level_name)
		if level == null:
			level_names.remove(i)
			continue
		levels.append(level)
		i += 1

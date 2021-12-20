extends Node

export var start_level = 0
export var level_dir = "res://levels/tst"

var levels: Array = []
var level: int = 0

func _ready():
#	levels = make_levels()
	levels = load_levels()
	if levels.size() == 0:
		quit()
		return
	level = start_level
	if set_initial_board_state():
		$Board.init_board()

func set_initial_board_state():
	var board_size = guess_board_size(levels[level].size())
	if board_size == 0:
		return false
	$Board.initial_state = levels[level]
	$Board.board_size = board_size
	return true

func guess_board_size(n):
	for i in range(3, n/2):
		if i * i == n:
			return i
	print("Couldn't determine board size: %d" % levels[level].size())
	print(levels[level])
	quit()
	return 0

func _input(event):
	if event is InputEventKey:
		$Board.reset_board()

func quit():
	get_tree().notification(MainLoop.NOTIFICATION_WM_QUIT_REQUEST)

func _on_Board_game_finished():
	level += 1
	if level == levels.size():
		quit()
		return
	set_initial_board_state()
	$Board.reset_board()

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
				level.append($Board.Piece.Hole)
			"*":
				level.append($Board.Piece.Boulder)
			"t":
				level.append($Board.Piece.Tractor)
			"#":
				level.append($Board.Piece.Block)
			_:
				print("Invalid level: %s" %  name)
				return null
	return level

func load_levels():
	var level_names = []
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
	var levels = []
	for level_name in level_names:
		var level = read_level(level_name)
		if level == null:
			return []
		levels.append(level)
	return levels

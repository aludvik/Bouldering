tool
extends EditorImportPlugin

var Piece = preload("res://level/Piece.gd")
var Level = preload("Level.gd")

func get_importer_name():
	return "level_importer"

func get_visible_name():
	return "Level"

func get_recognized_extensions():
	return ["lvl"]

func get_save_extension():
	return "res"

func get_resource_type():
	return "Resource"

func get_preset_count():
	return 1

func get_preset_name(i):
	return "Default"

func get_import_options(i):
	return []

func get_board_size(n):
	for i in range(3, n/2):
		if i * i == n:
			return i
	return -1

func import(source_file, save_path, options, platform_variants, gen_files):
	var file = File.new()
	var err = file.open(source_file, File.READ)
	if err != OK:
		return err
	var text = file.get_as_text()
	file.close()
	var cells = []
	for c in text:
		match c:
			"+", "-", "|", "\n":
				pass
			" ":
				cells.append(null)
			"O":
				cells.append(Piece.Hole)
			"*":
				cells.append(Piece.Boulder)
			"t":
				cells.append(Piece.Tractor)
			"#":
				cells.append(Piece.Block)
			_:
				printerr("Level `%s` contains invalid char `%s`: " %  [source_file, c])
				return ERR_PARSE_ERROR
	var size = get_board_size(cells.size())
	if size < 0:
		printerr("Level `%s` not square", source_file)
		return ERR_INVALID_DATA
	var level = Level.new()
	level.cells = cells
	level.size = size
	level.name = source_file.get_file().get_basename()
	var filename = save_path + "." + get_save_extension()
	return ResourceSaver.save(filename, level)

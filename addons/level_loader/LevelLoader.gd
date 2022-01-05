tool
extends EditorImportPlugin

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

func import(source_file, save_path, options, platform_variants, gen_files):
	var level = preload("Level.gd").new()
	level.cells = [
		null, null, null,
		null, null, null,
		null, null, null
	]
	var filename = save_path + "." + get_save_extension()
	return ResourceSaver.save(filename, level)

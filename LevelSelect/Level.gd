extends Node2D

export(int) var local_index

func update_for_page():
	var world_index = get_world_index()
	if world_index >= State.get_current_world_level_count():
		hide()
		return
	show()
	$Number.text = String(world_index + 1)
	var level_difficulty = State.get_level_data(world_index).difficulty
	get_node("Difficulty/" + State.world).frame = level_difficulty - 1
	show_completion(State.get_current_world_level_completion(world_index))

func show_completion(completion):
	var mod = Color(1.0, 1.0, 1.0, 1.0)
	if completion:
		mod = Color(1.0, 1.0, 1.0, 0.5)
	get_node("Header/" + State.world).modulate = mod
	get_node("Difficulty/" + State.world).modulate = mod
	$StartButton.modulate = mod
	$Lvl.modulate = mod
	$Number.modulate = mod

func get_world_index():
	return (State.page - 1) * 6 + local_index

func _on_page_changed():
	update_for_page()

func _on_StartButton_pressed():
	State.level = get_world_index()
	assert(get_tree().change_scene("res://Level/Level" + State.world + ".tscn") == OK)

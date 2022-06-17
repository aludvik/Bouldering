extends Node2D

export(int) var local_index

func update_for_page():
	var world_index = get_world_index()
	var levels_list = State.levels.get(State.world)
	if world_index >= levels_list.size():
		hide()
		return
	show()
	$Number.text = String(world_index + 1)
	var level_difficulty = levels_list[world_index].difficulty
	get_node("Difficulty/" + State.world).frame = level_difficulty - 1

func get_world_index():
	return (State.page - 1) * 6 + local_index

func _on_page_changed():
	update_for_page()

func _on_StartButton_pressed():
	State.level = get_world_index()
	assert(get_tree().change_scene("res://Level/Level.tscn") == OK)

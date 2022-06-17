extends TextureButton

export(String) var change_to

func _on_pressed():
	assert(get_tree().change_scene(change_to) == OK)

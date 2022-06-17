extends TextureButton

func _on_ReturnButton_pressed():
	State.page = 1
	assert(get_tree().change_scene("res://WorldSelect/WorldSelect.tscn") == OK)

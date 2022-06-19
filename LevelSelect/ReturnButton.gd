extends TextureButton

func _on_ReturnButton_pressed():
	assert(get_tree().change_scene("res://WorldSelect/WorldSelect.tscn") == OK)

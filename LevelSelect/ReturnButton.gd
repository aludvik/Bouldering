extends TextureButton

func _on_ReturnButton_pressed():
	State.page = 1
	get_tree().change_scene("res://WorldSelect/WorldSelect.tscn")

extends Control

func _input(event):
	if event is InputEventScreenTouch and event.pressed:
		var LevelSelect = load("res://level_select/LevelSelect.tscn")
		var level_select = LevelSelect.instance()
		var root = get_tree().get_root()
		root.remove_child(self)
		root.add_child(level_select)
		self.queue_free()

extends TextureButton

var sure = false

func _on_ResetProgressButton_pressed():
	if not sure:
		$Sure.show()
		sure = true
	else:
		State.reset_progress()

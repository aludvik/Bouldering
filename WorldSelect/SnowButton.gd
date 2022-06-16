extends TextureButton

func _ready():
	var lock_state = State.snow_world
	assert(lock_state != null)
	if lock_state == "unlocked":
		disabled = false
	elif lock_state == "locked":
		disabled = true
	else:
		assert(false)

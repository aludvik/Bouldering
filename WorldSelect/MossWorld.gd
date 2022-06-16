extends Node2D

func _ready():
	var lock_state = State.moss_world
	assert(lock_state != null)
	if lock_state == "unlocked":
		$Locked.hide()
		$Unlocked.show()
	elif lock_state == "locked":
		$Unlocked.hide()
		$Locked.show()
	else:
		assert(false)

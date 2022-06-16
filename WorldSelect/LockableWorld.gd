extends Node2D

export(String) var state_key

func _ready():
	var lock_state = State.get(state_key)
	assert(lock_state != null)
	if lock_state == "unlocked":
		$Locked.hide()
		$Unlocked.show()
		$EnterButton.disabled = false
	elif lock_state == "locked":
		$Unlocked.hide()
		$Locked.show()
		$EnterButton.disabled = true
	else:
		assert(false)

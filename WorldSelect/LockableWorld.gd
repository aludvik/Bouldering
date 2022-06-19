extends Node2D

export(String) var unlock_function

func _ready():
	if State.call(unlock_function):
		$Locked.hide()
		$Unlocked.show()
		$EnterButton.disabled = false
	else:
		$Unlocked.hide()
		$Locked.show()
		$EnterButton.disabled = true

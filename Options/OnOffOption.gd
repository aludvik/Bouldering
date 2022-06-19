extends Control

export(String) var state_key

func _ready():
	var state_value = State.get(state_key)
	assert(state_value != null)
	if state_value:
		show_on_button()
	else:
		show_off_button()

func show_on_button():
	$OnButton.show()
	$OffButton.hide()

func show_off_button():
	$OnButton.hide()
	$OffButton.show()

func _on_OnButton_pressed():
	State.set(state_key, false)
	State.save_state()
	show_off_button()

func _on_OffButton_pressed():
	State.set(state_key, true)
	State.save_state()
	show_on_button()

extends Control

# 2. Update state
# 3. Change the resolution

func _ready():
	make_active_button(State.resolution)

func change_resolution_to(size: int):
	assert(size >= 1 && size <= 4)
	make_active_button(size)
	State.resolution = size

func make_active_button(size: int):
	for i in range(1, 5):
		if i == size:
			set_button_activity(i, true)
		else:
			set_button_activity(i, false)

func set_button_activity(size: int, is_active: bool):
	var node = get_node(String(size) + "x")
	var active = node.get_node("ActiveButton")
	var inactive = node.get_node("InactiveButton")
	if is_active:
		active.show()
		inactive.hide()
	else:
		active.hide()
		inactive.show()

func _on_1x_pressed():
	change_resolution_to(1)

func _on_2x_pressed():
	change_resolution_to(2)

func _on_3x_pressed():
	change_resolution_to(3)

func _on_4x_pressed():
	change_resolution_to(4)

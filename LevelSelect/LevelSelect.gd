extends Node2D

signal page_changed

func _ready():
	set_world_color(State.world)
	set_page_label()
	emit_signal("page_changed")

func get_max_page():
	return ceil(State.levels.get(State.world).size() / 6.0)

func set_page_label():
	$Pager/Page.text = String(State.page) + "/" + String(get_max_page())

func set_world_color(world: String):
	var prefixes = [
		"Background",
		"TopLeft/Header",
		"TopLeft/Difficulty",
		"TopMid/Header",
		"TopMid/Difficulty",
		"TopRight/Header",
		"TopRight/Difficulty",
		"BottomLeft/Header",
		"BottomLeft/Difficulty",
		"BottomMid/Header",
		"BottomMid/Difficulty",
		"BottomRight/Header",
		"BottomRight/Difficulty"
	]
	for prefix in prefixes:
		for w in ["Rock", "Moss", "Snow"]:
			var node = get_node(prefix + "/" + w)
			if w == world:
				node.show()
			else:
				node.hide()

func _on_NextButton_pressed():
	if State.page >= get_max_page():
		return
	State.page += 1
	set_page_label()
	emit_signal("page_changed")

func _on_PrevButton_pressed():
	if State.page <= 1:
		return
	State.page -= 1
	set_page_label()
	emit_signal("page_changed")

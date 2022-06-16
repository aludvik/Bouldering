extends Node2D

var colored = [
	"Background",
	"TopLeft/Header",
	"TopLeft/Difficulty"
]

func _ready():
	set_world(State.world)

func set_world(world: String):
	for prefix in colored:
		set_world_color(world, prefix)

func set_world_color(world: String, prefix: String):
	for w in ["Rock", "Moss", "Snow"]:
		var node = get_node(prefix + "/" + w)
		if w == world:
			node.show()
		else:
			node.hide()

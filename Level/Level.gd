extends Node2D

onready var Tractor = $Tractor
onready var Boulder = $Boulder
onready var Buried = $Buried
onready var Stone = $Stone

export(int) var speed = 9

const tile_size = 16
const inputs = {
	"right": Vector2.RIGHT,
	"left": Vector2.LEFT,
	"up": Vector2.UP,
	"down": Vector2.DOWN
}

const rays = {
	"right": "Right",
	"left": "Left",
	"up": "Up",
	"down": "Down"
}

var moving = false
var filled = null

func _ready():
	remove_child($"4x4")
	remove_child($"5x5")

func _unhandled_input(event):
	if moving:
		return
	for dir in inputs.keys():
		if event.is_action_pressed("ui_" + dir):
			move(dir)

func move(dir):
	rotate_tractor(dir)
	var tractor_ray = Tractor.get_node(rays[dir])
	if tractor_ray.is_colliding():
		var collider = tractor_ray.get_collider()
		if collider.name == "Boulder":
			if !move_boulder(collider, dir):
				return
		else:
			return
	moving = true
	move_tractor(dir)

func move_boulder(boulder, dir):
	var boulder_ray = boulder.get_node(rays[dir])
	if boulder_ray.is_colliding():
		var collider = boulder_ray.get_collider()
		if collider.name != "Hole":
			return false
		filled = {"hole": collider, "boulder": boulder}
	move_tween(boulder, inputs[dir])
	return true

const rotations = {
	"up": 0,
	"right": 90,
	"down": 180,
	"left": 270
}

func rotate_tractor(dir):
	Tractor.get_node("AnimatedSprite").rotation_degrees = rotations[dir]

func move_tractor(dir):
	move_tween(Tractor, inputs[dir])

func move_tween(piece, dir):
	var tween = piece.get_node("Tween")
	tween.interpolate_property(
		piece,
		"position",
		piece.position,
		piece.position + dir * tile_size,
		1.0/speed,
		Tween.TRANS_SINE,
		Tween.EASE_IN_OUT)
	tween.start()

func _on_move_completed():
	moving = false
	if filled != null:
		var new_buried = Buried.duplicate()
		new_buried.position = filled["hole"].position
		remove_child(filled["hole"])
		remove_child(filled["boulder"])
		add_child(new_buried)

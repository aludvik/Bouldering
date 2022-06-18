extends Node2D

onready var Tractor = $Tractor
onready var Boulder = $Boulder
onready var Buried = $Buried
onready var Hole = $Hole
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
var remaining = 0

func _ready():
	load_level()

func load_level():
	var level = State.get_level_data()
	# Clear extra boards
	for size in range(4, 7):
		if size != level.size:
			remove_child(get_node(String(size)))
	# Clear game pieces
	for piece in [Boulder, Buried, Hole, Stone]:
		remove_child(piece)
	# Place game pieces based on level data
	var grid = get_node(String(level.size) + "/Grid")
	var origin = Vector2(grid.margin_left, grid.margin_top)
	for i in range(level.cells.size()):
		var cell = level.cells[i]
		if cell == "Tractor":
			Tractor.position = compute_piece_position(origin, level.size, i)
		elif cell != null:
			if cell == "Hole":
				remaining += 1
			var piece = get(cell).duplicate()
			piece.position = compute_piece_position(origin, level.size, i)
			add_child(piece)

func compute_piece_position(origin, grid_size, index):
	var col = index % grid_size
	var row = index / grid_size
# warning-ignore:integer_division
	var x_offset = col * tile_size + tile_size / 2
# warning-ignore:integer_division
	var y_offset = row * tile_size + tile_size / 2
	return origin + Vector2(x_offset, y_offset)

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
		if collider.is_in_group("movable"):
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
		if !collider.is_in_group("accepts_boulder"):
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

func handle_deferred_fill():
	if filled != null:
		var new_buried = Buried.duplicate()
		new_buried.position = filled["hole"].position
		remove_child(filled["hole"])
		remove_child(filled["boulder"])
		add_child(new_buried)
		filled = null
		remaining -= 1

func handle_level_complete():
	if remaining <= 0:
		$ResetButton.hide()
		$Success.show()

func _on_move_completed():
	moving = false
	handle_deferred_fill()
	handle_level_complete()

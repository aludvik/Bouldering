extends Node2D

onready var Tractor = $Tractor
onready var Boulder = $Pieces/Boulder.duplicate()
onready var Buried = $Pieces/Buried.duplicate()
onready var Hole = $Pieces/Hole.duplicate()
onready var Stone = $Pieces/Stone.duplicate()

onready var level = State.get_current_level_data()

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
	clear_extra_boards()
	load_level()

func load_level():
	remaining = 0
	clear_pieces()
	populate_pieces_from_level()

func clear_extra_boards():
	for size in range(4, 7):
		if size != level.size:
			var board = get_node(String(size))
			if board != null:
				remove_child(board)
				board.queue_free()

func clear_pieces():
	for piece in $Pieces.get_children():
		$Pieces.remove_child(piece)
		piece.queue_free()

func populate_pieces_from_level():
	var grid = get_node(String(level.size) + "/Grid")
	var origin = Vector2(grid.margin_left, grid.margin_top)
	for i in range(level.cells.size()):
		var cell = level.cells[i]
		if cell == "Tractor":
			Tractor.position = compute_piece_position(origin, level.size, i)
			rotate_tractor("up")
		elif cell != null:
			if cell == "Hole":
				remaining += 1
			var piece = create_piece(cell)
			piece.position = compute_piece_position(origin, level.size, i)
			$Pieces.add_child(piece)

func compute_piece_position(origin, grid_size, index):
	var col = index % grid_size
	var row = index / grid_size
# warning-ignore:integer_division
	var x_offset = col * tile_size + tile_size / 2
# warning-ignore:integer_division
	var y_offset = row * tile_size + tile_size / 2
	return origin + Vector2(x_offset, y_offset)

func create_piece(kind):
	return get(kind).duplicate()

func _unhandled_input(event):
	if moving or remaining == 0:
		return
	for dir in inputs.keys():
		if event.is_action_pressed("ui_" + dir):
			move(dir)

func move(dir):
	rotate_tractor(dir)
	var move = true
	var tractor_ray = Tractor.get_node(rays[dir])
	if tractor_ray.is_colliding():
		var collider = tractor_ray.get_collider()
		if collider.is_in_group("movable"):
			if !move_boulder(collider, dir):
				move = false
		else:
			move = false
	if move:
		moving = true
		move_tractor(dir)
	else:
		play_sound($BumpFx)

func move_boulder(boulder, dir):
	var boulder_ray = boulder.get_node(rays[dir])
	if boulder_ray.is_colliding():
		var collider = boulder_ray.get_collider()
		if !collider.is_in_group("accepts_boulder"):
			return false
		filled = {"hole": collider, "boulder": boulder}
	play_sound($PushFx)
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
		var new_buried = create_piece("Buried")
		new_buried.position = filled["hole"].position
		$Pieces.remove_child(filled["hole"])
		$Pieces.remove_child(filled["boulder"])
		$Pieces.add_child(new_buried)
		filled = null
		remaining -= 1
		play_sound($DropFx)

func handle_level_complete():
	if remaining <= 0:
		$ResetButton.hide()
		$Success.show()
		State.mark_current_level_complete()
		State.save_state()
		play_sound($CompleteFx)

func _on_move_completed():
	moving = false
	handle_deferred_fill()
	handle_level_complete()

func _on_ResetButton_pressed():
	load_level()
	play_sound($ResetFx)

func play_sound(node):
	if State.sfx:
		node.play()

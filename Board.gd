extends Node2D

export var board_size: int = 4
export var base_cell_size: int = 16
export var initial_state: Array = [
	null, null, Piece.Hole, null,
	Piece.Hole, Piece.Tractor, Piece.Boulder, Piece.Block,
	null, null, Piece.Boulder, null,
	Piece.Block, null, null, null
]

var scale_factor: int = 0

var Tractor = preload("res://Tractor.tscn")
var Hole = preload("res://Hole.tscn")
var Block = preload("res://Block.tscn")
var Boulder = preload("res://Boulder.tscn")
var BuriedBoulder = preload("res://BuriedBoulder.tscn")

enum Piece {Tractor, Hole, Block, Boulder}

func _ready():
	resize_background()
	scale_board()
	init_board()

# resize Background to board_size * base image size
func resize_background():
	$Background.margin_right = board_size * base_cell_size
	$Background.margin_bottom = board_size * base_cell_size

# scale board to fill screen while maintaining pixel-perfect scaling
func scale_board():
	scale_factor = compute_scale_factor()
	scale = Vector2(scale_factor, scale_factor)

func compute_scale_factor() -> int:
	# assert portrait mode
	var screen_width: int = get_viewport_rect().size.x
	var max_square_size: int = screen_width / board_size
	var scale_factor: int = max_square_size / base_cell_size
	return scale_factor

# instance all scenes based on initial_state
func init_board():
	assert(initial_state.size() == board_size * board_size)
	for row in range(board_size):
		for col in range(board_size):
			var piece = create_piece(initial_state[row * board_size + col])
			if piece == null:
				continue
			piece.position = Vector2(col * base_cell_size + base_cell_size / 2, row * base_cell_size  + base_cell_size / 2)
			self.add_child(piece)

func create_piece(object):
	match object:
		Piece.Tractor:
			return Tractor.instance()
		Piece.Hole:
			return Hole.instance()
		Piece.Block:
			return Block.instance()
		Piece.Boulder:
			return Boulder.instance()
	return null


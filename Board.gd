extends Node2D

signal game_finished

export var max_grid_size: Vector2 = Vector2(64, 64)
export var grid_size: Vector2 = Vector2(0, 0)
export var board_size: int = 0
export var base_cell_size: int = 16
export var initial_state: Array = []

var game_finished: bool = false
var tractor = null

var Tractor = preload("res://Tractor.tscn")
var Hole = preload("res://Hole.tscn")
var Block = preload("res://Block.tscn")
var Boulder = preload("res://Boulder.tscn")
var BuriedBoulder = preload("res://BuriedBoulder.tscn")

enum Piece {Tractor = 0, Hole = 1, Block = 2, Boulder = 3}

func _input(event):
	if is_square_clicked(event):
		if game_finished:
			return
		handle_click(to_local(event.position))

func clear_board():
	for piece in get_tree().get_nodes_in_group("pieces"):
		piece.queue_free()
	tractor = null

func reset_board():
	clear_board()
	init_board()

# resize GridTexture to board_size * base image size
func resize_background():
	$GridTexture.margin_right = board_size * base_cell_size
	$GridTexture.margin_bottom = board_size * base_cell_size

# scale board to fill screen while maintaining pixel-perfect scaling
func scale_board():
	# assert portrait mode
	var max_grid_width: float = max_grid_size.x
	var max_square_size: float = max_grid_width / board_size
	var scale_factor: int = max_square_size / base_cell_size
	scale = Vector2(scale_factor, scale_factor)
	var grid_width = scale_factor * base_cell_size * board_size
	grid_size = Vector2(grid_width, grid_width)

# instance all scenes based on initial_state
func init_board():
	assert(initial_state.size() == board_size * board_size)
	resize_background()
	scale_board()
	game_finished = false
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
			tractor = Tractor.instance()
			return tractor
		Piece.Hole:
			return Hole.instance()
		Piece.Block:
			return Block.instance()
		Piece.Boulder:
			var boulder = Boulder.instance()
			boulder.z_index += 1
			return boulder
	return null

func make_grid():
	var grid = []
	for row in range(board_size):
		for col in range(board_size):
			grid.append(null)
	for obstacle in get_tree().get_nodes_in_group("obstacles"):
		grid[position_to_index(obstacle.position)] = obstacle
	return grid

func is_square_clicked(event):
	if event is InputEventScreenTouch and event.pressed:
		var local = to_local(event.position)
		var local_grid_size = base_cell_size * board_size
		return (
			local.x > 0
			and local.y > 0
			and local.x < local_grid_size
			and local.y < local_grid_size
		)

func handle_click(position):
	if tractor.moving:
		return
	var target = position_to_index(position)
	var current = position_to_index(tractor.position)
	var grid = make_grid()
	if grid[target] == null:
		move_to_target(target, current, grid)
	elif grid[target].get_groups().has("boulders"):
		move_boulder(target, current, grid)

func move_to_target(target, current, grid):
	var graph = make_graph(grid)
	var path = find_path(current, target, graph)
	if path != null:
		var pixel_path = convert_path_cells_to_pixels(path)
		tractor.move_along_path(pixel_path)

func move_boulder(target, current, grid):
	var dir = border_direction(current, target)
	if dir == null:
		return
	if !can_move_boulder(target, dir, grid):
		return
	var boulder_target
	match dir:
		Direction.DOWN:
			boulder_target = target + board_size
		Direction.UP:
			boulder_target = target - board_size
		Direction.RIGHT:
			boulder_target = target + 1
		Direction.LEFT:
			boulder_target = target - 1
	var boulder = grid[target]
	boulder.move_to(index_to_position(boulder_target))
	tractor.move_to(index_to_position(target))
	if grid[boulder_target] == null:
		return
	if !grid[boulder_target].get_groups().has("holes"):
		return
	var hole = grid[boulder_target]
	yield(boulder, "finished_moving")
	boulder.queue_free()
	hole.queue_free()
	var buried = BuriedBoulder.instance()
	buried.position = index_to_position(boulder_target)
	add_child(buried)
	if get_tree().get_nodes_in_group("holes").size() == 1:
		game_finished = true
		emit_signal("game_finished")

func convert_path_cells_to_pixels(path):
	var new_path = []
	for idx in path:
		new_path.append(index_to_position(idx))
	return new_path

# Grid - Array of objects. Every cell has either an object, representing an
# obstacle, or null, representing no obstacle.

# Graph - A 2D array of integers. Each cell of the outer array represents a
# node in the graph. Each inner array represents all of the edges from one
# node of the graph to another node by including it's index in the outer array.

# Grid to Graph - The grid is converted to a graph by assigning graph indices to
# every cell using the formula `row * width + column` and by inserting an edge
# `j` into the graph at index `i` if `i` is a cell bordering `j` and there is no
# obstacle at cell `j`.

# Find a path between start and end on the graph provided. If a path exists,
# it returns an array of node ids from the graph representing the path from
# start to end, not including the start node id and including the end id. If
# no path exists, returns nil.

func find_path(start, end, graph):
	var dist = []
	var prev = []
	var queue = []
	for i in range(graph.size()):
		dist.append(INF)
		prev.append(null)
		queue.append(i)
	dist[start] = 0
	while !queue.empty():
		var min_idx = 0
		for i in range(queue.size()):
			if dist[queue[i]] < dist[queue[min_idx]]:
				min_idx = i
		var u = queue[min_idx]
		queue.remove(min_idx)
		for v in graph[u]:
			if queue.has(v):
				var alt = dist[u] + 1
				if alt < dist[v]:
					dist[v] = alt
					prev[v] = u
	if prev[end] == null: # no path exists
		return null
	var path = []
	var u = end
	while prev[u] != null:
		path.push_front(u)
		u = prev[u]
	return path

func coord_to_index(row, col):
	return row * board_size + col

func index_to_row(idx: int) -> int:
	return idx / board_size

func index_to_col(idx: int) -> int:
	return idx % board_size

func index_to_position(idx: int):
	var x = (idx % board_size) * base_cell_size + base_cell_size / 2
	var y = (idx / board_size) * base_cell_size + base_cell_size / 2
	return Vector2(x, y)

func position_to_index(position: Vector2):
	var coord = (position / base_cell_size).floor()
	return coord_to_index(coord.y, coord.x)

enum Direction {UP, DOWN, LEFT, RIGHT}

# Return the direction to cell b from cell a if they are bordering or NONE if
# they are not bordering.
func border_direction(a: int, b: int):
	var row_a = index_to_row(a)
	var row_b = index_to_row(b)
	var col_a = index_to_col(a)
	var col_b = index_to_col(b)
	if row_a - row_b == -1 and col_a == col_b:
		return Direction.DOWN
	if row_a - row_b == 1 and col_a == col_b:
		return Direction.UP
	if row_a == row_b and col_a - col_b == -1:
		return Direction.RIGHT
	if row_a == row_b and col_a - col_b == 1:
		return Direction.LEFT
	return null

func can_move_boulder(idx: int, direction: int, grid) -> bool:
	var row = index_to_row(idx)
	var col = index_to_col(idx)
	match direction:
		Direction.DOWN:
			return row != board_size - 1 and accepts_boulder(row + 1, col, grid)
		Direction.UP:
			return row != 0 and accepts_boulder(row - 1, col, grid)
		Direction.RIGHT:
			return col != board_size - 1 and accepts_boulder(row, col + 1, grid)
		Direction.LEFT:
			return col != 0 and accepts_boulder(row, col - 1, grid)
	return false

func accepts_boulder(row, col, grid):
	var object = grid[coord_to_index(row, col)]
	return object == null or object.get_groups().has("holes")

# Convert the grid into a graph using the Grid to Graph correspondence.
func make_graph(grid):
	var graph = []
	for idx in range(grid.size()):
		var edges = []
		var row = index_to_row(idx)
		var col = index_to_col(idx)
		var up = coord_to_index(row - 1, col)
		if row != 0 and grid[up] == null:
			edges.append(up)
		var down = coord_to_index(row + 1, col)
		if row != board_size - 1 and grid[down] == null:
			edges.append(down)
		var left = coord_to_index(row, col - 1)
		if col != 0 and grid[left] == null:
			edges.append(left)
		var right = coord_to_index(row, col + 1)
		if col != board_size - 1 and grid[right] == null:
			edges.append(right)
		graph.append(edges)
	return graph

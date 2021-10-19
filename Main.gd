extends Node2D

# The number of cells in the gird
export var board_size: int = 4

var BuriedBoulder = preload("res://BuriedBoulder.tscn")
var screen_size
var square_size

func _ready():
	screen_size = get_viewport_rect().size
	square_size = screen_size / board_size
	OS.set_window_size(screen_size * 4)

func make_grid():
	var grid = []
	for row in range(board_size):
		for col in range(board_size):
			grid.append(null)
	for obstacle in get_tree().get_nodes_in_group("obstacles"):
		grid[position_to_index(obstacle.position)] = obstacle
	return grid

func is_square_clicked(event):
	return (
		event is InputEventScreenTouch
		and event.pressed
		and event.position.x > 0
		and event.position.x < screen_size.x
		and event.position.y > 0
		and event.position.y < screen_size.y
	)

func _input(event):
	if is_square_clicked(event):
		handle_click(event.position)

func handle_click(position):
	var target = position_to_index(position)
	var tractor = position_to_index($Tractor.position)
	var grid = make_grid()
	if grid[target] == null:
		move_to_target(target, tractor, grid)
	elif grid[target].get_groups().has("boulders"):
		move_boulder(target, tractor, grid)

func move_to_target(target, tractor, grid):
	var graph = make_graph(grid)
	var path = find_path(tractor, target, graph)
	if path != null:
		var pixel_path = convert_path_cells_to_pixels(path)
		$Tractor.move_along_path(pixel_path)

func move_boulder(target, tractor, grid):
	var dir = border_direction(tractor, target)
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
	$Tractor.move_to(index_to_position(target))
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
	return Vector2(idx % board_size, idx / board_size) * square_size + square_size / 2

func position_to_index(position: Vector2):
	var coord = (position / square_size).floor()
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

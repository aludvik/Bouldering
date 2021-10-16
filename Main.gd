extends Node2D

# The number of cells in the gird
export var board_size: int = 4

var grid
var screen_size
var square_size

enum {BOULDER}

func _ready():
	grid = [	null, null, null, null,
			null, null, BOULDER, null,
			null, null, BOULDER, null,
			null, null, null, null]
	screen_size = get_viewport_rect().size
	square_size = screen_size / board_size

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
		move_to_position(event.position)

func move_to_position(position):
	var tractor = position_to_index($Tractor.position)
	var target = position_to_index(position)
	var graph = make_graph(grid)
	var path = find_path(tractor, target, graph)
	if path != null:
		var pixel_path = convert_path_cells_to_pixels(path)
		$Tractor.move_along_path(pixel_path)

func convert_path_cells_to_pixels(path):
	var new_path = []
	for idx in path:
		var vec = index_to_vector(idx)
		var pixel_vec = vec * square_size + square_size / 2
		new_path.append(pixel_vec)
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

func index_to_vector(idx: int):
	return Vector2(idx % board_size, idx / board_size)

func position_to_index(position: Vector2):
	var coord = (position / square_size).floor()
	return coord_to_index(coord.y, coord.x)

# Convert the grid into a graph using the Grid to Graph correspondence.
func make_graph(grid):
	var graph = []
	for idx in range(grid.size()):
		var edges = []
		var row = idx / board_size
		var col = idx % board_size
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

extends Node2D

# The number of cells in the gird
export var board_size: int = 4

var path
var grid
var graph
var tractor
var screen_size
var square_size

enum {BOULDER}

func _ready():
	path = []
	grid = [	[null, null, null, null],
			[null, null, BOULDER, null],
			[null, null, BOULDER, null],
			[null, null, null, null]]
	tractor = 5
	graph = make_graph(grid)
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
#		build_path(event.position)
		move_to_position(event.position)

func move_to_position(position):
	var center = (position / square_size).floor()
	var target = coord_to_index(center.y, center.x)
	var path = find_path(tractor, target, graph)
	if path != null:
		var pixel_path = convert_path_cells_to_pixels(path)
		$Tractor.move_along_path(pixel_path)
	tractor = target

func build_path(position):
	var center = (position / square_size).floor() * square_size + square_size / 2
	if path.has(center):
		$Tractor.move_along_path(path)
	else:
		path.push_front(center)

func convert_path_cells_to_pixels(path):
	var new_path = []
	for idx in path:
		var vec = index_to_vector(idx)
		var pixel_vec = vec * square_size + square_size / 2
		new_path.append(pixel_vec)
	return new_path

# Grid - row-major order 2D array of objects. Every cell has either an object,
# representing an obstacle, or null, representing no obstacle.

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

# Convert the grid into a graph using the Grid to Graph correspondence.
func make_graph(grid):
	var graph = []
	for row in range(grid.size()):
		for col in range(grid[row].size()):
			var edges = []
			if row != 0 and grid[row - 1][col] == null:
				edges.append(coord_to_index(row - 1, col))
			if row != grid.size() - 1 and grid[row + 1][col] == null:
				edges.append(coord_to_index(row + 1, col))
			if col != 0 and grid[row][col - 1] == null:
				edges.append(coord_to_index(row, col - 1))
			if col != grid.size() - 1 and grid[row][col + 1] == null:
				edges.append(coord_to_index(row, col + 1))
			graph.append(edges)
	return graph

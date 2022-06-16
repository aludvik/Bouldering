extends Control

var levels: Array = []
onready var container = $MarginContainer/ScrollContainer/HBoxContainer/Levels
var Tile = preload("res://level_select/Tile.tscn")

func _ready():
	levels = preload("res://LevelList.gd").new().levels
	for lvl in levels:
		State.completion.push_back(false)
	for idx in range(levels.size()):
		add_tile(idx, levels[idx])

func add_tile(idx, level):
	var tile = Tile.instance()
	tile.set_difficulty(level.difficulty)
	tile.set_index(idx)
	tile.set_completion(State.completion[idx])
	tile.connect("selected", self, "_on_Tile_selected")
	container.add_child(tile)

func _on_Tile_selected(index):
	var Level = load("res://level/Level.tscn")
	var level = Level.instance()
	level.level = levels[index]
	level.index = index
	var root = get_tree().get_root()
	root.remove_child(self)
	root.add_child(level)
	self.queue_free()

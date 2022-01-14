extends Control

var levels: Array = []
onready var container = $ScrollContainer/HBoxContainer/Levels
var Tile = preload("res://level_select/Tile.tscn")

func _ready():
	levels = preload("res://LevelList.gd").new().levels
	for idx in range(levels.size()):
		add_tile(idx)

func add_tile(idx):
	var tile = Tile.instance()
	tile.set_difficulty(1)
	tile.set_index(idx)
	tile.set_completion(false)
	tile.connect("selected", self, "_on_Tile_selected")
	container.add_child(tile)

func _on_Tile_selected(index):
	var Level = load("res://level/Level.tscn")
	var level = Level.instance()
	level.level = levels[index]
	var root = get_tree().get_root()
	root.remove_child(self)
	root.add_child(level)
	self.queue_free()

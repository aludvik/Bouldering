extends Node

# Level
var world: String = "Rock"
var page: int = 1 # 1-indexed
var level: int = 4 # 0-indexed

# Progress
var moss_world: String = "unlocked"
var snow_world: String = "unlocked"

# Settings
var music: bool = true
var sfx: bool = true
var resolution: int = 2 setget resolution_set

func _ready():
	resolution_set(resolution)

func resolution_set(size):
	resolution = size
	var viewport = get_tree().get_root()
	OS.window_size = viewport.size * size

func reset_progress():
	print("Reset progress")

var levels = preload("res://LevelList.gd").new()

func get_level_data():
	return levels.get(world)[level]

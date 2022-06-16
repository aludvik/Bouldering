extends Node

var moss_world: String = "unlocked"
var snow_world: String = "locked"

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

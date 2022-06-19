extends Node

# Level
var world: String = "Rock"
var page: int = 1 # 1-indexed
var level: int = 4 # 0-indexed

# Progress
var completion = {
	"Rock": {},
	"Moss": {},
	"Snow": {}
}

# Settings
var music: bool = true
var sfx: bool = true
var resolution: int = 2 setget resolution_set

func _ready():
	resolution_set(resolution)

# Helper functions
func resolution_set(size):
	resolution = size
	var viewport = get_tree().get_root()
	OS.window_size = viewport.size * size

func reset_progress():
	print("Reset progress")

func get_current_level_data():
	return levels[world][level]

func get_level_data(l):
	return levels[world][l]

func get_current_world_level_count():
	return levels[world].size()

func get_current_world_level_completion(lvl):
	return completion[world].has(lvl)

func mark_current_level_complete():
	completion[world][level] = true

func is_moss_world_unlocked():
	return levels["Rock"].size() == completion["Rock"].size()

func is_snow_world_unlocked():
	return levels["Moss"].size() == completion["Moss"].size()

var levels = {
	"Rock": [
		preload("res://Levels/04-01-v0.lvl"),
		preload("res://Levels/04-02-v0.lvl"),
		preload("res://Levels/04-03-v0.lvl"),
		preload("res://Levels/04-06-v1.lvl"),
		preload("res://Levels/04-11-v1.lvl"),
		preload("res://Levels/04-12-v2.lvl"),
		preload("res://Levels/04-14-v1.lvl"),
		preload("res://Levels/04-16-v2.lvl"),
		preload("res://Levels/04-17-v3.lvl"),
		preload("res://Levels/05-03-v1.lvl"),
		preload("res://Levels/04-04-v0.lvl"),
		preload("res://Levels/04-08-v0.lvl"),
		preload("res://Levels/04-07-v1.lvl"),
		preload("res://Levels/04-13-v2.lvl"),
		preload("res://Levels/04-15-v1.lvl"),
		preload("res://Levels/05-04-v2.lvl"),
		preload("res://Levels/04-05-v0.lvl"),
	],
	"Moss": [
		preload("res://Levels/05-07-v1.lvl"),
		preload("res://Levels/05-05-v2.lvl"),
		preload("res://Levels/04-09-v1.lvl"),
		preload("res://Levels/05-06-v2.lvl"),
		preload("res://Levels/05-10-v3.lvl"),
		preload("res://Levels/08.05-107.1350-v2.lvl"),
		preload("res://Levels/09.05-110.5091-v1.lvl"),
		preload("res://Levels/12.05-105.4254-v1.lvl"),
		preload("res://Levels/39.05-110.4012-v1.lvl"),
		preload("res://Levels/43.05-111.1459-v2.lvl"),
		preload("res://Levels/46.05-111.1364-v1.lvl"),
		preload("res://Levels/04-10-v2.lvl"),
		preload("res://Levels/05-01-v3.lvl"),
	],
	"Snow": [
		preload("res://Levels/25.05-105.4237-v2.lvl"),
		preload("res://Levels/40.05-111.3981-v2.lvl"),
		preload("res://Levels/13.05-112.3466-v3.lvl"),
		preload("res://Levels/14.05-16-v4.lvl"),
		preload("res://Levels/17.05-11-v3.lvl"),
		preload("res://Levels/19.05-109.724-v3.lvl"),
		preload("res://Levels/41.05-112.3870-v2.lvl"),
		preload("res://Levels/47.05-111.3490-v2.lvl"),
		preload("res://Levels/48.05-15-v3.lvl"),
		preload("res://Levels/52.05-105.4088-v3.lvl"),
		preload("res://Levels/53.05-08-v2.lvl"),
		preload("res://Levels/61.05-112.5033-v3.lvl"),
		preload("res://Levels/64.05-09-v2.lvl"),
		preload("res://Levels/65.05-106.2775-v2.lvl"),
		preload("res://Levels/70.05-109.393-v2.lvl"),
		preload("res://Levels/77.05-108.1983-v2.lvl"),
		preload("res://Levels/78.05-112.1436-v2.lvl"),
		preload("res://Levels/80.05-108.2160-v2.lvl"),
		preload("res://Levels/91.05-20-v2.lvl"),
		preload("res://Levels/93.05-106.1694-v5.lvl"),
		preload("res://Levels/97.05-106.1696-v4.lvl")
	]
}

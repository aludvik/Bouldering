extends Control

signal selected

var difficulty_textures: Array = [
	preload("res://level_select/LevelDifficulty1.png"),
	preload("res://level_select/LevelDifficulty2.png"),
	preload("res://level_select/LevelDifficulty3.png"),
	preload("res://level_select/LevelDifficulty4.png"),
	preload("res://level_select/LevelDifficulty5.png"),
	preload("res://level_select/LevelDifficulty6.png"),
	preload("res://level_select/LevelDifficulty7.png")
]

var index = -1

func set_index(idx: int):
	assert(idx >= 0 && idx < 999)
	index = idx
	$Number.text = String(index + 1)

func set_completion(completed: bool):
	var mod
	if completed:
		mod = Color(1.0, 1.0, 1.0, 0.5)
	else:
		mod = Color(1.0, 1.0, 1.0, 1.0)
	$Header.modulate = mod
	$Lvl.modulate = mod
	$Number.modulate = mod
	$Difficulty.modulate = mod
	$Start.modulate = mod

func set_difficulty(difficulty: int):
	assert(difficulty > 0 && difficulty <= 7)
	$Difficulty.texture = difficulty_textures[difficulty - 1]

func _on_Start_pressed():
	emit_signal("selected", index)

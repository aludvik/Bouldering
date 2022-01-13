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

var incomplete_texture = preload("res://level_select/LevelCheck1.png")
var completed_texture = preload("res://level_select/LevelCheck2.png")

func set_number(number: int):
	assert(number > 0 && number <= 999)
	$Number.text = String(number)

func set_difficulty(difficulty: int):
	assert(difficulty > 0 && difficulty <= 7)
	$Difficulty.texture = difficulty_textures[difficulty - 1]

func set_completion(completion: bool):
	$Completed.texture = completed_texture if completion else incomplete_texture


func _on_Start_pressed():
	emit_signal("selected")

extends Node2D

func _on_RockButton_pressed():
	go_to_world("Rock")

func _on_MossButton_pressed():
	go_to_world("Moss")

func _on_SnowButton_pressed():
	go_to_world("Snow")

func go_to_world(world: String):
	State.world = world
	get_tree().change_scene("res://LevelSelect/LevelSelect.tscn")

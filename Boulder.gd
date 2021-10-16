extends Sprite

export var speed = 1

var should_move
var target

func _ready():
	should_move = false
	target = Vector2()

func _process(delta):
	if should_move:
		if position.is_equal_approx(target):
			position = target
			should_move = false
			return
		position = position.move_toward(target, delta * speed)

func move_to(new_target):
	should_move = true
	target = new_target
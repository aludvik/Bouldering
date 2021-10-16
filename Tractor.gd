extends AnimatedSprite

signal finished_moving

export var speed = 100

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
			emit_signal("finished_moving")
			return
		rotation = position.angle_to_point(target) - PI / 2
		position = position.move_toward(target, delta * speed)

func move_to(new_target):
	should_move = true
	target = new_target

func move_along_path(path):
	if path.empty():
		return
	var new_target = path.pop_front()
	move_to(new_target)
	yield(self, "finished_moving")
	move_along_path(path)

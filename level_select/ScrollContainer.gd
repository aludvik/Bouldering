extends ScrollContainer

func _ready() -> void:
	disable_scrollbars() #Run the function to disable the scrollbars

func disable_scrollbars():
	var invisible_scrollbar_theme = Theme.new()
	var empty_stylebox = StyleBoxEmpty.new()
	invisible_scrollbar_theme.set_stylebox("scroll", "VScrollBar", empty_stylebox)
	invisible_scrollbar_theme.set_stylebox("scroll", "HScrollBar", empty_stylebox)
	get_v_scrollbar().theme = invisible_scrollbar_theme
	get_h_scrollbar().theme = invisible_scrollbar_theme

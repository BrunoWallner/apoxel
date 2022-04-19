extends Spatial

onready var backend = self.get_node("../BackendHandle");

func _ready():
	get_tree().set_auto_accept_quit(false);

func _notification(what):
	if what == MainLoop.NOTIFICATION_WM_QUIT_REQUEST:
		print("exiting");
		backend.terminate(); # very important
		self.get_tree().quit();

extends Node

onready var tcp = self.get_node("../TcpWrapper");

func _ready():
	tcp.register("Luca");
	
	tcp.move(Vector3(0.123456789, 0, 0));
	
	tcp.connect("Error", self, "handle_error");
	tcp.connect("Token", self, "handle_token");
	tcp.connect("ChunkUpdate", self, "handle_chunk");
	
func handle_error(error: String):
	#print("error: ", error);
	pass
	
func handle_token(token: Array):
	#print("token: ", token);
	tcp.token = token;
	tcp.login();

func handle_chunk(position: Array, chunk: Array):
	var pos: Vector3 = Vector3(position[0], position[1], position[2]);
	

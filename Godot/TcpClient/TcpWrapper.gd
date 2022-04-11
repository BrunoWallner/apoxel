extends Node;

# string
signal ChunkUpdate(position, chunk);
# array
signal Token(token);
# string
signal Error(kind);

export var host: String = "0.0.0.0:8000";

onready var client: Node = self.get_node("TcpClient");

var token: Array = [];

func _ready():
	client.establish_connection(host);
	
	client.connect("Event", self, "handle_event");
	
func connection_established() -> bool:
	return client.connection_established();
	
func register(name: String):
	var string = '{"Register":{"name":"' + name + '"}}';
	client.send(string);
	
func login():
	if !token.empty():
		var token_string: String = str(token).replace(" ", "");

		var string = '{"Login":{"token":' + token_string + '}}';
		client.send(string);
	

func move(pos: Vector3):
	var x = str(pos.x).pad_decimals(10);
	var y = str(pos.y).pad_decimals(10);
	var z = str(pos.z).pad_decimals(10);
	var string = '{"Move":{"coord":[' + x + "," + y + "," + z + ']}}';
	
	client.send(string);
	
func handle_event(event: String):
	var type: String = event.split('"', true, 2)[1];
	match type:
		"Error":
			var kind: String = JSON.parse(event).result.Error;
			self.emit_signal("Error", kind);
		"Token":
			print(JSON.parse(event).result);
			var token: Array = JSON.parse(event).result.Token;
			self.emit_signal("Token", token);
			
		"ChunkUpdate":
			var result = JSON.parse(event).result;
			#print(result);
			var coord = result.ChunkUpdate.coord;
			var data: Array = result.ChunkUpdate.data;
			self.emit_signal("ChunkUpdate", coord, data);
		_:
			print("invalid event type");
	


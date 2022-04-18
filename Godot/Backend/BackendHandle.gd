extends Node;

export var host: String = "0.0.0.0:8000";

onready var client: Node = self.get_node("Backend");

var token: Array = [];

const TOKEN_LENGTH: int = 16;

func _ready():
	client.establish_connection(host);
	
	# login if token is stored
	var file = File.new();
	if file.file_exists("user://token.dat"):
		file.open("user://token.dat", File.READ);
		token = file.get_buffer(TOKEN_LENGTH);
		file.close();
	else:
		self.register("Luca");
	
	self.login();
	
func _process(_dt: float):
	# fetching events	
	var event = client.fetch_event();
	if event:
		var type = event[0];
		match type:
			"Token":
				# login 
				token = event[1][0];
				self.login();
				
				var file = File.new();
				file.open("user://token.dat", File.WRITE);
				file.store_buffer(token);
				file.close();

			"Error":
				var error = event[1][0];
				match error:
					"Login":
						print("invalid login token");
						var dir = Directory.new();
						dir.remove("user://token.dat");
					"Register":
						print("user already registered");
						
						

func handle_chunk_update(chunk):
	self.emit_signal("ChunkUpdate", chunk);
	
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

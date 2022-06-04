extends Node;

export var host: String = "0.0.0.0:8000";
export var login_name: String = "Luca";

onready var client: Node = self.get_node("Backend");

var logged_in: bool = false;
var token: Array = [];

const TOKEN_LENGTH: int = 16;

func terminate():
	client.terminate();
	

func _ready():
	client.establish_connection(host);
	
	# login if token is stored
	var file = File.new();
	if file.file_exists("user://token.dat"):
		file.open("user://token.dat", File.READ);
		token = file.get_buffer(TOKEN_LENGTH);
		file.close();
		self.login();
	else:
		self.register(login_name);
	
func _process(_dt: float):
	if Input.is_action_just_pressed("ui_cancel"):
		#self.terminate();
		pass
		
	# fetching events	
	var event = client.fetch_event();
	if event:
		var type = event[0];
		match type:
			"Token":
				# login
				# assuming that the token the server gave is correct
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
						self._ready();
						self.logged_in = false;
					"Register":
						print("user already registered");
						
					"ConnectionReset":
						print("connection reset");
						self.logged_in = false;
						
						

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
		self.logged_in = true;
	

func move(pos: Vector3):
	var x = str(pos.x).pad_decimals(10);
	var y = str(pos.y).pad_decimals(10);
	var z = str(pos.z).pad_decimals(10);
	var string = '{"Move":{"coord":[' + x + "," + y + "," + z + ']}}';
	
	client.send(string);

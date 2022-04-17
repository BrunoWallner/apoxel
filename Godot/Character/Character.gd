extends KinematicBody

onready var character = get_node("../Character");
onready var tcp = get_node("../TcpWrapper");

export var SPEED: float = 1.0;
export var GRAVITY: float = 10.0;
export var FRICTION: float = 1.25;
export var JUMP: float = 10.0;
export var IN_AIR_MOVEMENT_CAP: float = 0.33;

var velocity: Vector3 = Vector3.ZERO;
var in_air: bool = true;

var mouse_captured: bool = false;
var mouse_sens = 1.0
var mouse_relative_x = 0
var mouse_relative_y = 0

# mouse input
func _input(event):         
	if event is InputEventMouseMotion && mouse_captured:
		self.rotation_degrees.y -= event.relative.x * mouse_sens / 18
		$Camera.rotation_degrees.x -= event.relative.y * mouse_sens / 18
		$Camera.rotation_degrees.x = clamp($Camera.rotation_degrees.x, -90, 90)
		
		mouse_relative_x = clamp(event.relative.x, -50, 50)
		mouse_relative_y = clamp(event.relative.y, -50, 10)

# called from _physics_process
# handles velocity
func handle_input(_dt: float) -> Vector3:
	var input_dir: Vector3 = Vector3.ZERO;
	# directions
	if Input.is_action_pressed("move_forward"):
		input_dir += -self.global_transform.basis.z;
	if Input.is_action_pressed("move_backwards"):
		input_dir += self.global_transform.basis.z;
	if Input.is_action_pressed("move_left"):
		input_dir += -self.global_transform.basis.x;
	if Input.is_action_pressed("move_right"):
		input_dir += self.global_transform.basis.x;
		
	# jumping
	if !in_air && Input.is_action_just_pressed("jump"):
		input_dir.y = JUMP;
		in_air = true;
		
	# mouse capture
	if Input.is_action_pressed("ui_cancel"):
		Input.set_mouse_mode(Input.MOUSE_MODE_VISIBLE);
		mouse_captured = false;
	if Input.is_action_pressed("window_click"):
		Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED);
		mouse_captured = true;
		
	# diagonal movement speed limit
	# additional movement cap if in air
	var cap = 1.0;
	if in_air:
		cap = IN_AIR_MOVEMENT_CAP;
		
	var vxz: Vector2 = Vector2(input_dir.x, input_dir.z).normalized() * cap * SPEED;
	
	return Vector3(vxz.x, input_dir.y, vxz.y);

func _physics_process(dt: float):
	var input_dir = handle_input(dt);
	velocity += input_dir;
	
	# gravity
	if velocity.y > -GRAVITY:
		velocity.y -= 0.5;
		
	var _none = self.move_and_slide(velocity, Vector3.UP, true);
	if self.is_on_floor():
		velocity.y = 0.0;
		in_air = false;
		
	# friction
	velocity.x /= FRICTION;
	velocity.z /= FRICTION;
	
	# sync position with server
	tcp.move(self.transform.origin);

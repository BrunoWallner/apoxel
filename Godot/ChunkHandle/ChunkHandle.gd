extends Node

onready var tcp = self.get_node("../TcpWrapper/TcpClient");
onready var mesh_gen = self.get_node("MeshGenerator");

func _process(_dt: float):
	# queuing up mesh generation
	var chunk_updates: Array = tcp.fetch_chunk_update();
	for chunk_update in chunk_updates:
		mesh_gen.queue_chunk(chunk_update);

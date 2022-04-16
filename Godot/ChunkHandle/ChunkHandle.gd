extends Node

onready var tcp = self.get_node("../TcpWrapper/TcpClient");
onready var mesh_gen = self.get_node("MeshGenerator");

func _process(_dt: float):
	# queuing up mesh generation
	var chunk_update = tcp.fetch_chunk_update();
	if chunk_update:
		mesh_gen.queue_chunk(chunk_update);
		
	# non blocking fetching of meshed chunk
	# TODO create ArrayMesh in rust
	var meshed = mesh_gen.fetch_mesh();
	if meshed:
		# mesh array
		var arr = [];
		arr.resize(Mesh.ARRAY_MAX);

		# extraction of data		
		var coord: Array = meshed[0];

		var verts: PoolVector3Array = meshed[1];
		var uvs: PoolVector2Array = meshed[2];
		var normals: PoolVector3Array = meshed[3];
		var indices: PoolIntArray = meshed[4];
		
		# when chunk is only air do not build
		if verts.size() != 0:
			# Assign arrays to mesh array.
			arr[Mesh.ARRAY_VERTEX] = verts
			arr[Mesh.ARRAY_TEX_UV] = uvs
			arr[Mesh.ARRAY_NORMAL] = normals
			arr[Mesh.ARRAY_INDEX] = indices
		
			var array_mesh = ArrayMesh.new();
			array_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arr);
			var mesh_collision_shape = array_mesh.create_trimesh_shape();
			
			var collision_shape: CollisionShape = CollisionShape.new();
			collision_shape.shape = mesh_collision_shape;
			
			var static_body: StaticBody = StaticBody.new();
			static_body.add_child(collision_shape);
			
			var mesh_instance = MeshInstance.new();
			mesh_instance.mesh = array_mesh;
			mesh_instance.add_child(static_body);
			
			# for position
			var spatial = Spatial.new();
			spatial.translate(Vector3(coord[0], coord[1], coord[2]));
			#spatial.rotate( Vector3(x: 0.0, y: 1.0, z: 0.0), 0.0 );
			spatial.rotate(Vector3(0.0, 1.0, 0.0), 0.0);
			spatial.add_child(mesh_instance);
				
			self.add_child(spatial);

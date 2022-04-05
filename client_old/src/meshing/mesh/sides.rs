const TEXTURE_SIZE: f32 = 10.0;
const WHOLE_TEXTURE_SIZE: f32 = 64.0; // square

const UV_SIZE: f32 = TEXTURE_SIZE / WHOLE_TEXTURE_SIZE;

pub fn push_uvs(uvs: &mut Vec<[f32; 2]>, block_category: (u32, u32)) {
    let uv_start_x: f32 = block_category.1 as f32 * UV_SIZE + 1.0 / WHOLE_TEXTURE_SIZE;
    let uv_start_y: f32 = block_category.0 as f32 * UV_SIZE + 1.0 / WHOLE_TEXTURE_SIZE;

    let uv_end_x: f32 = (block_category.1 + 1) as f32 * UV_SIZE - 1.0 / WHOLE_TEXTURE_SIZE;
    let uv_end_y: f32 = (block_category.0 + 1) as f32 * UV_SIZE - 1.0 / WHOLE_TEXTURE_SIZE;

    uvs.push([ uv_start_x, uv_start_y ]);
    uvs.push([ uv_end_x, uv_start_y ]);
    uvs.push([ uv_start_x, uv_end_y ]);
    uvs.push([ uv_end_x, uv_end_y]);
}

pub fn left(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ -1.0, 0.0, 0.0 ];


    positions.push([ x, y, z ]);
    positions.push([ x, y, z + 1.0 ]);
    positions.push([ x, y + 1.0, z ]);
    positions.push([ x, y + 1.0, z + 1.0 ]);

    for _ in 0..4 {
        normals.push(norms);
    }

    // creates indices
    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 3) as u32 );
    indices.push( (positions_len + 0) as u32 );

    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 3) as u32 );
    indices.push( (positions_len + 2) as u32 );
}

pub fn right(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ 1.0, 0.0, 0.0 ];


    positions.push([ x + 1.0, y, z ]);
    positions.push([ x + 1.0, y, z + 1.0 ]);
    positions.push([ x + 1.0, y + 1.0, z ]);
    positions.push([ x + 1.0, y + 1.0, z + 1.0 ]);

    for _ in 0..4 {
        normals.push(norms);
    }

    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 3) as u32 );
    indices.push( (positions_len + 1) as u32 );

    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 3) as u32 );
    indices.push( (positions_len + 0) as u32 );
}

pub fn back(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ 0.0, 0.0, -1.0 ];


    positions.push([ x, y, z]);
    positions.push([ x + 1.0, y, z ]);
    positions.push([ x, y + 1.0, z ]);
    positions.push([ x + 1.0, y + 1.0, z ]);

    for _ in 0..4 {
        normals.push(norms);
    }
    
    // creates indices
    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 1) as u32 );

    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 3) as u32 );
}

pub fn front(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ 0.0, 0.0, 1.0 ];


    positions.push([ x, y, z + 1.0 ]);
    positions.push([ x + 1.0, y, z + 1.0 ]);
    positions.push([ x, y + 1.0, z + 1.0 ]);
    positions.push([ x + 1.0, y + 1.0, z + 1.0 ]);

    for _ in 0..4 {
        normals.push(norms);
    }

    // creates indices
    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 2) as u32 );

    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 3) as u32 ); 
}

pub fn top(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ 0.0, 1.0, 0.0 ];


    positions.push([ x, y + 1.0, z ]);
    positions.push([ x, y + 1.0, z + 1.0 ]);
    positions.push([ x + 1.0, y + 1.0, z ]);
    positions.push([ x + 1.0, y + 1.0, z + 1.0] );

    for _ in 0..4 {
        normals.push(norms);
    }

    // creates indices
    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 3) as u32 );

    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 3) as u32 ); 
}


pub fn bottom(
    x: f32,
    y: f32,
    z: f32,
    
    positions: &mut Vec<[f32; 3]>, 
    normals: &mut Vec<[f32; 3]>, 
    
    indices: &mut Vec<u32>
) {
    let norms = [ 0.0, -1.0, 0.0 ];


    positions.push([ x, y, z ]);
    positions.push([ x, y, z + 1.0 ]);
    positions.push([ x + 1.0, y, z ]);
    positions.push([ x + 1.0, y, z + 1.0 ]);

    for _ in 0..4 {
        normals.push(norms);
    }

    // creates indices
    let positions_len = positions.len() - 4;
    indices.push( (positions_len + 0) as u32 );
    indices.push( (positions_len + 2) as u32 );
    indices.push( (positions_len + 1) as u32 );


    indices.push( (positions_len + 3) as u32 );
    indices.push( (positions_len + 1) as u32 );
    indices.push( (positions_len + 2) as u32 );
}
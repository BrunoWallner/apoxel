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
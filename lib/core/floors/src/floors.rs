use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

// have a list of all the floors, so we can know which are drawn and which are not
// each entity has a floors component which states which floors it overlaps with
// something that moves needs to have a system that tracks it's floor so that it can be removed
// have a system that looks for floors that are being removed or added and draws/removes items

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Floor {
    pub object_ids: Vec<usize>,
    pub floors_overlapped: Vec<usize>,
}

impl Floor {
    pub fn new(object_ids: Vec<usize>, floors_overlapped: Vec<usize>) -> Self {
        Floor {
            object_ids,
            floors_overlapped,
        }
    }

    pub fn contains(&self, floor: &usize) -> bool {
        self.floors_overlapped.contains(floor)
    }
}

#[derive(Clone, Debug)]
pub struct FloorBoundaries {
    pub floor_number: usize,
    pub half_size: Vector2<f32>,
    pub position: Vector2<f32>,
}

impl Default for FloorBoundaries {
    fn default() -> Self {
        FloorBoundaries {
            floor_number: 0,
            half_size: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
        }
    }
}

impl FloorBoundaries {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct FloorsDrawn {
    pub floor_boundaries: Vec<FloorBoundaries>,
    pub rendered_floors: Vec<usize>,
    pub pending_removal: Vec<(usize, f64)>,
    pub rendered_ids: Vec<usize>,
}

impl Default for FloorsDrawn {
    fn default() -> Self {
        Self {
            floor_boundaries: Vec::new(),
            rendered_floors: Vec::new(),
            pending_removal: Vec::new(),
            rendered_ids: Vec::new(),
        }
    }
}

impl FloorsDrawn {
    pub fn add_boundary(
        &mut self,
        floor_number: usize,
        position: Vector2<f32>,
        width: f32,
        height: f32,
    ) {
        self.floor_boundaries.push(FloorBoundaries {
            floor_number,
            position,
            half_size: Vector2::new(width / 2., height / 2.),
        });
    }

    pub fn find_floors(&self, position: Vector2<f32>, width: f32, height: f32) -> Vec<usize> {
        let mut floors: Vec<usize> = Vec::new();
        for boundary in &self.floor_boundaries {
            // determine if they overlap
            let overlaps = (boundary.position.x - position.x).abs()
                <= (boundary.half_size.x + width / 2.).abs()
                && (boundary.position.y - position.y).abs()
                    <= (boundary.half_size.y + height / 2.).abs();
            if overlaps {
                floors.push(boundary.floor_number);
            }
        }
        floors
    }
}

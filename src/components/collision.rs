use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone)]
pub struct GenericBox {
    pub half_size: Vector2<f32>,
    pub position: Vector2<f32>,
    pub old_position: Vector2<f32>,
}

impl Default for GenericBox {
    fn default() -> Self {
        Self {
            half_size: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
            old_position: Vector2::new(0., 0.),
        }
    }
}

impl GenericBox {
    pub fn new(width: f32, height: f32) -> Self {
        GenericBox {
            half_size: Vector2::new(width / 2., height / 2.),
            ..GenericBox::default()
        }
    }
}

#[derive(Debug)]
pub struct CollideeDetails {
    pub name: String,
    pub position: Vector2<f32>,
    pub half_size: Vector2<f32>,
    pub correction: f32,
    pub velocity: f32,
    pub collided_with_name: String,
    pub collided_with_velocity: f32,
    pub is_rideable: bool,
}

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct ProximityDetails {
    pub name: String,
    pub other_name: String,
    pub distance: Vector2<f32>,
    pub approaching: bool,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Proximity {
    pub min_distance: f32,
    pub details: Vec<ProximityDetails>,
}

const DEFAULT_PROXIMITY_PADDING: f32 = 3.0;

impl Default for Proximity {
    fn default() -> Self {
        Proximity {
            min_distance: DEFAULT_PROXIMITY_PADDING,
            details: Vec::new(),
        }
    }
}

impl Proximity {
    pub fn reset_details(&mut self) {
        self.details = Vec::new();
    }

    pub fn add_proximity_details(
        &mut self,
        name_a: String,
        name_b: String,
        collider_a: &Collider,
        collider_b: &Collider,
        velocity_a: Vector2<f32>,
        use_hit_box: bool,
    ) {
        let min_distance = self.min_distance;
        let (self_box, other_box) = if use_hit_box {
            (&collider_a.hit_box, &collider_b.hit_box)
        } else {
            (&collider_a.bounding_box, &collider_b.bounding_box)
        };

        // if we are approaching x coords and y is close
        let x_diff = ((self_box.half_size.x + other_box.half_size.x).abs()
            - (self_box.position.x - other_box.position.x).abs())
        .abs();
        let y_diff = ((self_box.half_size.y + other_box.half_size.y).abs()
            - (self_box.position.y - other_box.position.y).abs())
        .abs();

        // check that it's not overlapping, but that it's within the min_distance
        if !collider_a.is_overlapping_with(collider_b, use_hit_box)
            && x_diff <= min_distance
            && y_diff <= min_distance
        {
            let self_center_x = self_box.position.x + self_box.half_size.x;
            let other_center_x = other_box.position.x + other_box.half_size.x;
            let self_center_y = self_box.position.y + self_box.half_size.y;
            let other_center_y = other_box.position.y + other_box.half_size.y;
            let approaching: bool = (self_center_x < other_center_x && velocity_a.x > 0.)
                || (self_center_x > other_center_x && velocity_a.x < 0.)
                || (self_center_y < other_center_y && velocity_a.y > 0.)
                || (self_center_y > other_center_y && velocity_a.y < 0.);
            self.details.push(ProximityDetails {
                name: name_a,
                other_name: name_b,
                distance: Vector2::new(x_diff, y_diff),
                approaching,
            });
        }
    }
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collidee {
    pub horizontal: Option<CollideeDetails>,
    pub vertical: Option<CollideeDetails>,
}

impl Default for Collidee {
    fn default() -> Self {
        Self {
            horizontal: None,
            vertical: None,
        }
    }
}

impl Collidee {
    pub fn set_collidee_details(
        &mut self,
        name: String,
        collided_with_name: String,
        collider_a: &Collider,
        collider_b: &Collider,
        velocity_a: Vector2<f32>,
        velocity_b: Vector2<f32>,
        use_hit_box: bool,
    ) {
        let (box_a, box_b) = if use_hit_box {
            (&collider_a.hit_box, &collider_b.hit_box)
        } else {
            (&collider_a.bounding_box, &collider_b.bounding_box)
        };

        let mut correction = Vector2::new(0., 0.);

        let speed_sum = Vector2::new(
            (velocity_a.x - velocity_b.x).abs(),
            (velocity_a.y - velocity_b.y).abs(),
        );
        let speed_ratio_a = Vector2::new(velocity_a.x / speed_sum.x, velocity_a.y / speed_sum.y);
        let speed_ratio_b = Vector2::new(velocity_b.x / speed_sum.x, velocity_b.y / speed_sum.y);

        let min_safe_distance = Vector2::new(
            box_a.half_size.x + box_b.half_size.x,
            box_a.half_size.y + box_b.half_size.y,
        );
        let overlap = Vector2::new(
            min_safe_distance.x - (box_a.position.x - box_b.position.x).abs(),
            min_safe_distance.y - (box_a.position.y - box_b.position.y).abs(),
        );
        let x_overlapped = (box_a.old_position.x - box_b.old_position.x).abs()
            < box_a.half_size.x + box_b.half_size.x;
        let y_overlapped = (box_a.old_position.y - box_b.old_position.y).abs()
            < box_a.half_size.y + box_b.half_size.y;

        let same_direction = velocity_a.x * velocity_b.x > 0.;
        let faster = speed_ratio_a.x.abs() > speed_ratio_b.x.abs();
        if (y_overlapped || overlap.x.abs() <= overlap.y.abs()) && !x_overlapped {
            correction.x = if (faster || !same_direction) && !speed_ratio_a.x.is_nan() {
                overlap.x * speed_ratio_a.x
            } else {
                0.
            };
            // No correction (correction = 0.) is required if collider is slower
            // and both bodies are moving in the same direction
            self.horizontal = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.x,
                velocity: velocity_a.x,
                collided_with_name,
                collided_with_velocity: velocity_b.x,
                is_rideable: collider_b.is_rideable,
            });
        } else if x_overlapped && y_overlapped {
            // Might happen when an entity is added at run time.
            // As per the current game design, no correction (correction = 0.) is required for this scenario
            // This might have to be changed in future
            self.horizontal = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.x,
                velocity: velocity_a.x,
                collided_with_name,
                collided_with_velocity: velocity_b.x,
                is_rideable: collider_b.is_rideable,
            });
        } else {
            correction.y = if !speed_ratio_a.y.is_nan() {
                overlap.y * speed_ratio_a.y
            } else {
                0.
            };
            self.vertical = Some(CollideeDetails {
                name,
                position: box_b.position,
                half_size: box_b.half_size,
                correction: correction.y,
                velocity: velocity_a.y,
                collided_with_name,
                collided_with_velocity: velocity_b.y,
                is_rideable: collider_b.is_rideable,
            });
        }
    }
}

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub bounding_box: GenericBox,
    pub bounding_box_offset: Vector2<f32>,
    pub hit_box: GenericBox,
    pub hit_box_offset: Vector2<f32>,
    pub on_ground: bool,
    pub on_elevator: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
    pub is_collidable: bool,
    pub is_rideable: bool,
    pub allow_proximity: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            bounding_box: GenericBox::default(),
            bounding_box_offset: Vector2::new(0., 0.),
            hit_box: GenericBox::default(),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            on_elevator: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: true,
            is_rideable: false,
            allow_proximity: true,
        }
    }
}

impl Collider {
    pub fn new(width: f32, height: f32) -> Self {
        Collider {
            bounding_box: GenericBox::new(width, height),
            hit_box: GenericBox::new(width, height),
            ..Collider::default()
        }
    }

    pub fn set_hit_box_position(&mut self, velocity: Vector2<f32>) {
        let hbox_position = &mut self.hit_box.position;
        let bbox_position = self.bounding_box.position;
        hbox_position.x = if velocity.x >= 0. {
            bbox_position.x + self.hit_box_offset.x
        } else {
            bbox_position.x - self.hit_box_offset.x
        };
        hbox_position.y = if velocity.y >= 0. {
            bbox_position.y + self.hit_box_offset.y
        } else {
            bbox_position.y - self.hit_box_offset.y
        }
    }

    pub fn is_overlapping_with(&self, other: &Collider, use_hit_box: bool) -> bool {
        let (self_box, other_box) = if use_hit_box {
            (&self.hit_box, &other.hit_box)
        } else {
            (&self.bounding_box, &other.bounding_box)
        };
        ((self_box.position.x - other_box.position.x).abs()
            <= (self_box.half_size.x + other_box.half_size.x).abs()
            && (self_box.position.y - other_box.position.y).abs()
                <= (self_box.half_size.y + other_box.half_size.y).abs())
    }
}

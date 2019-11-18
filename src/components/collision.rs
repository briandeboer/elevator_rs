use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Boundary {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Boundary {
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

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

#[derive(Clone, Component)]
#[storage(DenseVecStorage)]
pub struct Collider {
    pub bounding_box: GenericBox,
    pub hit_box: GenericBox,
    pub hit_box_offset: Vector2<f32>,
    pub on_ground: bool,
    pub hit_box_offset_front: f32,
    pub hit_box_offset_back: f32,
    pub is_collidable: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            bounding_box: GenericBox::default(),
            hit_box: GenericBox::default(),
            hit_box_offset: Vector2::new(0., 0.),
            on_ground: false,
            hit_box_offset_front: 0.,
            hit_box_offset_back: 0.,
            is_collidable: true,
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
        ((self_box.position.x - other_box.position.x).abs() <= (self_box.half_size.x + other_box.half_size.x).abs() &&
            (self_box.position.y - other_box.position.y).abs() <= (self_box.half_size.y + other_box.half_size.y).abs())
    }
}
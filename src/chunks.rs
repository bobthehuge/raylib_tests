use raylib::{ 
    core:: {
        texture::Image,
    },
    ffi::{ 
        Vector2, 
        Vector3,
    },
    prelude::{
        Color,
    },
};

pub struct Chunk {
    coordinates: Vector3,
    position: Vector2,
    size: Vector2,
    color: Color,
}

pub struct Map {
    chunks: Vec<Chunk>,
    dimensions: Vector2,
    resolution: f32,
}

impl Map {
    pub fn new(width: f32, height: f32, resolution: f32) -> Map {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(
            (width * height) as usize
        );

        for i in 0..(width/resolution) as u32 {
            for j in 0..(height/resolution) as u32 {
                chunks.push(Chunk{
                    coordinates: Vector3{
                        x: i as f32, 
                        y: j as f32, 
                        z: 0.0,
                    },
                    position: Vector2{
                        x: (i as f32) * resolution, 
                        y: (j as f32) * resolution, 
                    },
                    size: Vector2{
                        x: resolution,
                        y: resolution,
                    },
                    color: {
                        match (i+j).checked_rem(2) {
                            Some(rem) => {
                                if rem == 0 {
                                    Color::BLACK
                                } else {
                                    Color::WHITE
                                }
                            },
                            None => Color::BLACK
                        }
                    }
                });
            }
        }

        let dimensions = Vector2{ x: width, y: height };

        return Map {
            chunks: chunks,
            dimensions: dimensions,
            resolution: resolution,
        };
    }

    pub fn render_to_image(&self) -> Image {
        let mut image = Image::gen_image_color(
            self.dimensions.x as i32,
            self.dimensions.y as i32,
            Color::WHITE,
        );

        for chunk in &self.chunks {
            image.draw_rectangle(
                chunk.position.x as i32, 
                chunk.position.y as i32, 
                chunk.size.x as i32,
                chunk.size.y as i32,
                chunk.color,
            );
        }

        image
    }
}

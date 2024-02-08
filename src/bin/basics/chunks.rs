use raylib::{ 
    core:: {
        texture::Image,
    },
    math::{ 
        Vector3,
    },
    prelude::{
        Color,
    },
};

pub struct Chunk {
    coords: Vector3,
    pos: Vector3,
    size: Vector3,
    color: Color,
}

pub struct Map {
    chunks: Vec<Chunk>,
    size: Vector3,
    res: f32,
}

impl Map {
    pub fn new(dims: Vector3, resolution: f32) -> Map {
        let mut chunks: Vec<Chunk> = Vec::with_capacity(
            (dims.x * dims.y * dims.z) as usize
        );

        for i in 0..(dims.z/resolution) as u32 {
            for j in 0..(dims.y/resolution) as u32 {
                for k in 0..(dims.x/resolution) as u32 {
                    chunks.push(Chunk{
                        coords: Vector3::new(
                            k as f32, 
                            j as f32, 
                            i as f32,
                        ),
                        pos: Vector3::new(
                            (k as f32) * resolution, 
                            (j as f32) * resolution, 
                            (i as f32) * resolution, 
                        ),
                        size: Vector3::new(
                            resolution,
                            resolution,
                            resolution,
                        ),
                        color: {
                            match ((i+j+k)).checked_rem(2) {
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
        }

        return Map {
            chunks: chunks,
            size: dims,
            res: resolution,
        };
    }

    pub fn render_to_image(&self) -> Image {
        let mut image = Image::gen_image_color(
            self.size.x as i32,
            self.size.y as i32,
            Color::WHITE,
        );

        for chunk in &self.chunks {
            image.draw_rectangle(
                chunk.pos.x as i32, 
                chunk.pos.y as i32, 
                chunk.size.x as i32,
                chunk.size.y as i32,
                chunk.color,
            );
        }

        image
    }
}

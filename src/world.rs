/*
 * manages the world state, like loaded chunks and blocks
 */
use crate::render::Instance;

const AIR: u32 = 0;
const MOSS: u32 = 1;

pub struct Chunk {
  pub blocks: [[[u32; 16]; 256]; 16], // chunks are 16x256x16 [z][y][x]
  pub x: i32,
  pub y: i32
}

impl Chunk {

  /**
   * create a test chunk
   */
  pub fn test () -> Chunk {

    // create an empty chunk
    let mut blocks = [[[AIR; 16]; 256]; 16];

    // place a few moss blocks
    for x in 0..16 {

      for z in 0..16 {

        blocks[z][0][x] = MOSS;

      }

    }

    Chunk {
      blocks,
      x: 0,
      y: 0
    }

  }

  pub fn gen_instances (&self) -> Vec<Instance> {

    let mut instances = vec![];

    for x in 0..16 {

      for y in 0..256 {

        for z in 0..16 {

          let block = self.blocks[z][y][x];

          if block != AIR {
            
            let x: i32 = self.x * 16 + x as i32;
            let y: i32 = y as i32;
            let z: i32 = self.y * 16 + z as i32;

            let instance = Instance {
              pos: [x, y, z],
              block
            };

            instances.push(instance);
          }

        }

      }

    }

    instances

  }

}

pub struct World {
  pub loaded_chunks: Vec<Chunk>
}

impl World {

  /**
   * create a test world
   */
  pub fn test () -> World {

    // create a test chunk
    let chunk = Chunk::test();
    // and add it to the world
    let loaded_chunks = vec![chunk];

    return World {
      loaded_chunks
    }
    
  }

  /**
   * generate instances for each block in the world
   */
  pub fn gen_instances (&self) -> Vec<Instance> {

    self.loaded_chunks.iter().flat_map(|chunk| chunk.gen_instances()).collect()

  }

}
use std::fs::File;

/*
 * manages the world state, like loaded chunks and blocks
 */
use crate::render::Instance;
use mca_parser::{nbt::BlockState, Chunk as McaChunk, Region};

const AIR: u32 = 0;
const MOSS: u32 = 1;

pub struct Block {
  pub id: u32,
  pub x: i32,
  pub y: i32,
  pub z: i32
}

pub struct Chunk {
  pub blocks: Vec<Block>, // chunks are 16x384x16 [z][y][x]
  pub x: i32,
  pub z: i32
}

impl Chunk {

  /**
   * create a test chunk
   */
  pub fn test () -> Chunk {

    // create an empty chunk
    let mut blocks: Vec<Block> = vec![];

    // place a few moss blocks
    for x in 0..16 {

      for z in 0..16 {

        blocks.push(Block {
          id: MOSS,
          x: x as i32,
          y: 0,
          z: z as i32
        });

      }

    }

    blocks.push(Block {
      id: MOSS,
      x: 1,
      y: 1,
      z: 1
    });

    Chunk {
      blocks,
      x: 0,
      z: 0
    }

  }

  /**
   * create an instance vector for rending from each block in this chunk
   */
  pub fn gen_instances (&self) -> Vec<Instance> {

    self.blocks.iter().map(|block| {
      Instance {
        pos: [self.x * 16 + block.x, block.y, self.z * 16 + block.z],
        block: block.id
      }
    }).collect()

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
   * parse a world from a file
   */
  pub fn parse_world (file: &mut File) -> World {

    // create a list of chunks
    let mut chunks: Vec<Chunk> = vec![];

    // load the region
    let region = Region::from_reader(file).unwrap();

    // for now, just load the first chunk
    let chunk = region.get_chunk(0, 0).unwrap();

    if let Some(chunk) = chunk {
      // parse the raw chunk data into structured NBT format
      let parsed = chunk.parse().unwrap();
      // create a list of blocks for this chunk
      let mut blocks: Vec<Block> = vec![];
      
      // chunks are broken up into 16x16 sections
      println!("section count: {}", parsed.sections.len());
      parsed.sections.iter().for_each(|section| {
        let y_offset = section.y as i32 * 16;

        let block_states = section.block_states.as_ref().unwrap();
        let palette = block_states.palette.clone();
        let data = block_states.data.clone();
        if data.is_some() {

          // there are multiple blocks in this section
          let section_blocks = data.unwrap();

          println!("there are {} blocks in this section", section_blocks.len());

          section_blocks.iter().enumerate().for_each(|(i, block)| {
            let x = i as i32 & 0xf;
            let z = (i as i32 >> 4) & 0xf;
            let y = y_offset + (i >> 8) as i32;
            
            // only keep going if block is actually in palette
            if (*block as usize) != 0 {

              println!("new block at {} {} {}", x, y, z);
              // add it to the chunk
              blocks.push(Block {
                id: MOSS,
                x,
                y,
                z
              });
  
            }
  
          });

        } else {
          // all blocks in this section are the same
          let block = palette.get(0).unwrap();

          if (block.name.key != "air") {
            // add it to the chunk
            for i in 0..4096 {
              let x = i as i32 & 0xf;
              let z = (i as i32 >> 4) & 0xf;
              let y = y_offset + (i >> 8) as i32;
              blocks.push(Block {
                id: MOSS,
                x,
                y,
                z
              });
            }
          }
        }

      });

      println!("chunk at {} {}", parsed.x_pos, parsed.z_pos);

      // define the chunk and add it to the world
      chunks.push(Chunk {
        blocks,
        x: parsed.x_pos,
        z: parsed.z_pos
      });

    } else {
        // If the chunk is None, it has not been generated
        println!("Chunk has not been generated.");
    }

    println!("loaded {} blocks", chunks.get(0).unwrap().blocks.len());

    World {
      loaded_chunks: chunks
    }

  }

  /**
   * generate instances for each block in the world
   */
  pub fn gen_instances (&self) -> Vec<Instance> {

    self.loaded_chunks.iter().flat_map(|chunk| chunk.gen_instances()).collect()

  }

}
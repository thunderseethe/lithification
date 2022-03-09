use anyhow::*;
use morton_encoding::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Block {
    id: u32,
}

impl Block {
    pub fn new(id: u32) -> Self {
        Block { id }
    }
    fn to_bytes(&self) -> Vec<u8> {
        bytemuck::bytes_of(&self.id).to_owned()
    }

    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < 4 {
            return Err(anyhow!("Not enough bytes to construct Block ID"));
        }
        let id: u32 = *bytemuck::from_bytes(&bytes[0..4]);
        Ok(Block { id })
    }
}

type BlockArray = [Block; 32 * 32 * 32];

#[derive(Clone, Debug)]
pub struct Chunk(BlockArray);
impl Chunk {
    pub fn from_fn(f: impl Fn(u8, u8, u8) -> Block) -> Self {
        let mut arr: [Block; 32 * 32 * 32] = [Block { id: 0 }; 32 * 32 * 32];

        for x in 0..32 {
            for y in 0..32 {
                for z in 0..32 {
                   arr[morton_encode([x, y, z]) as usize] = f(x, y, z);
                }
            }
        }

        Chunk(arr)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.iter().flat_map(|block| {
            bytemuck::bytes_of(&block.id).iter().cloned()
        })
        .collect::<Vec<_>>()
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != 32 * 32 * 32 * 4 {
            return Err(anyhow!("Not enough bytes to construct a chunk"));
        }

        let blocks: Vec<Block> = bytes.chunks(4)
            .map(|slice| Block::from_bytes(slice))
            .collect::<anyhow::Result<Vec<Block>>>()?;

        blocks.try_into()
            .map(Chunk)
            .map_err(|_| anyhow!("Failed to make a Chunk from bytes"))
    }
}

#[derive(Debug)]
pub struct Message {
    pub tag: String,
    pub bytes: Vec<u8>,
}
impl Message {

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.tag.len() + self.bytes.len() + 4);
        let tag_len = self.tag.len() as u32;

        bytes.extend(bytemuck::bytes_of(&tag_len));
        bytes.extend(self.tag.as_bytes());
        bytes.extend(&self.bytes[..]);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < 4 {
            return Err(anyhow!("Expected at least 4 bytes of input"));
        }
        let len: u32 = *bytemuck::from_bytes(&bytes[0..4]);
        log::info!("tag length {}", len);
        let msg_offset: usize = 4 + len as usize;
        if bytes.len() < msg_offset {
            return Err(anyhow!("Expected message to contain tag string"));
        }
        let tag = String::from_utf8(bytes[4..msg_offset].to_owned())?;
        log::info!("tag {:?}", tag);
        let payload = bytes[msg_offset..].to_owned();
        
        Ok(Message {
            tag,
            bytes: payload,
        })
    }
}

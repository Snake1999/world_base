// http://wiki.vg/Protocol#Position
#[derive(Debug)]
struct BlockPos (u64);

impl BlockPos {
    fn get_block_x(&self) -> i32 {
        let x = (self.0 >> 38) as i32;
        if x >= 2i32.pow(25) {x - 2i32.pow(26)}
        else {x}
    }
    fn get_block_y(&self) -> i16 {
        let y = ((self.0 >> 26) & 0xFFF) as i16;
        if y >= 2i16.pow(11) {y - 2i16.pow(12)}
        else {y}
    }
    fn get_block_z(&self) -> i32 {
        let z = (self.0 << 38 >> 38) as i32;
        if z >= 2i32.pow(25) {z - 2i32.pow(26)}
        else {z}
    }
}

trait ToBlockPos {
    fn to_block_pos(&self) -> BlockPos;
}

impl ToBlockPos for (i32, i16, i32) {
    fn to_block_pos(&self) -> BlockPos {
        BlockPos(
            (((self.0 & 0x3FFFFFF) as u64)<< 38)|
                (((self.1 & 0xFFF) as u64) << 26)|
                (self.2 & 0x3FFFFFF) as u64
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_block_pos() {
        use ToBlockPos;
        let pos = vec![
            ((1234, -56, 789), 339470456980245u64),
            ((-33554432, -2048, -33554432), 9223372174327283712u64),
            ((33554431, 2047, 33554431), 9223371899382267903u64),
            ((0, 0, 0), 0u64),
        ];
        for i in pos.iter() {
            let pos = i.0.to_block_pos();
            assert_eq!(pos.0, i.1);
            assert_eq!(i.0, (pos.get_block_x(), pos.get_block_y(), pos.get_block_z()));
        }
    }
}

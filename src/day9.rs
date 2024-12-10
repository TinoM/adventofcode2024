use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let disk:Vec<u8> = input.bytes().map(|b|b-b'0').collect();
    let mut checksum = 0;
    // eprintln!("{:?}", disk);
    let mut diskvec = vec![];
    let mut freeblocks = vec![];
    let mut blockidx:usize = 0;
    for i in 0..disk.len() {
        if i%2 == 0 {
            diskvec.push(Block{fileid:i/2,length:disk[i],idx:blockidx});
        } else if disk[i] != 0 {
            freeblocks.push((disk[i],blockidx));
        }
        blockidx += disk[i] as usize;
    }

    // eprintln!("{:?}", diskvec);
    // eprintln!("{:?}", freeblocks);
    let mut moved_blocks = vec![];
    while let Some((freeblock_size,freeblock_startidx)) = freeblocks.first_mut() {
        if let Some(fileblock) = diskvec.last_mut() {
            if fileblock.idx < *freeblock_startidx {
                // eprintln!("Free block at end of disk");
                break;
            }
            //fileblock completely fits into freeblock
            if fileblock.length <= *freeblock_size {
                // eprintln!("{:?} moved to {}", fileblock, *freeblock_startidx);
                *freeblock_size -= fileblock.length;
                moved_blocks.push(Block{fileid:fileblock.fileid,length:fileblock.length,idx:*freeblock_startidx});
                *freeblock_startidx += fileblock.length as usize; 
                diskvec.pop();
                
                    
            } else {
                // eprintln!("Length {} of {:?} moved to {}",freeblock_size, fileblock, freeblock_startidx);
                moved_blocks.push(Block{fileid:fileblock.fileid,length:*freeblock_size,idx:*freeblock_startidx});
                fileblock.length -= *freeblock_size;
                // fileblock.idx += *freeblock_size as usize;
                *freeblock_size = 0;
            }
            if *freeblock_size == 0 {
                freeblocks.remove(0);
            }
        }
        // eprintln!("{:?}\n{:?}\n", diskvec,freeblocks);
    }
    // eprintln!("\n{:?}\n", diskvec);
    // eprintln!("{:?}", moved_blocks);
    for block in diskvec.iter().chain(moved_blocks.iter()) {
        for i in block.idx..block.idx+block.length as usize {
            // eprintln!("{} at {} => {}",block.fileid,i,i*block.fileid);
            checksum += i*block.fileid;
        }
    }
    checksum
}

#[derive(Debug)]
struct Block {
    fileid: usize,
    length: u8,
    idx: usize
}


#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let disk:Vec<u8> = input.bytes().map(|b|b-b'0').collect();
    let mut checksum = 0;
    // eprintln!("{:?}", disk);
    let mut diskvec = vec![];
    let mut freeblocks = vec![];
    let mut blockidx:usize = 0;
    for i in 0..disk.len() {
        if i%2 == 0 {
            diskvec.push(Block{fileid:i/2,length:disk[i],idx:blockidx});
        } else if disk[i] != 0 {
            freeblocks.push((disk[i],blockidx));
        }
        blockidx += disk[i] as usize;
    }

    // eprintln!("{:?}", diskvec);
    // eprintln!("{:?}", freeblocks);
    let mut moved_blocks = vec![];
    let mut file_to_analyze = diskvec.len()-1;
    while let Some(fileblock) = diskvec.get_mut(file_to_analyze) {
        for (freeblock_size,freeblock_startidx) in freeblocks.iter_mut().filter(|b|b.0 > 0) {
            if *freeblock_size >= fileblock.length {
                // eprintln!("{:?} moved to {}", fileblock, *freeblock_startidx);
                *freeblock_size -= fileblock.length;
                moved_blocks.push(Block{fileid:fileblock.fileid,length:fileblock.length,idx:*freeblock_startidx});
                *freeblock_startidx += fileblock.length as usize; 
                fileblock.length = 0;
                break;
            }
        }
        if file_to_analyze == 0 {
            break;
        }
        file_to_analyze -= 1;
    }
    // eprintln!("\n{:?}\n", diskvec);
    // eprintln!("{:?}", moved_blocks);
    for block in diskvec.iter().chain(moved_blocks.iter()).filter(|b|b.length > 0) {
        for i in block.idx..block.idx+block.length as usize {
            // eprintln!("{} at {} => {}",block.fileid,i,i*block.fileid);
            checksum += i*block.fileid;
        }
    }
    checksum
}



#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 2858);
    }
}
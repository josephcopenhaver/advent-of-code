use std::error::Error;

const INPUT: &str = include_str!("../../../input.txt");

fn main() -> Result<(), Box<dyn Error>> {
    // alternating sequences of numbers 0-9
    // each even idx is a new file, the value indicates the block length
    // each odd idx represents a block of free space which may be zero (no gap)
    //
    // this means each file cannot be more than 9 blocks in length
    //
    // each tail element valid file block is shifted to the leftmost empty block until contiguous
    //
    // no need to optimize for moving multiple segments at once ( (n * (n+1) / 2) + k*k )
    // since the values are so low and input length is not terribly long
    // at least for my hardware and lifespan...

    // assumes input has odd number of characters
    let mut nums = Vec::<i8>::with_capacity(INPUT.len());
    for v in INPUT.as_bytes() {
        nums.push((*v - b'0') as i8);
    }
    let mut offsets = Vec::<i32>::with_capacity((INPUT.len() + 1) / 2);
    let mut space_used: Vec<u8> = vec![0; INPUT.len() / 2];
    {
        let mut idx = 0;
        let mut i = 0;
        while i < nums.len() - 1 {
            offsets.push(idx);
            idx += (nums[i] + nums[i + 1]) as i32;
            i += 2;
        }
    }

    let mut moving_file_id = nums.len() / 2;

    let mut sum = 0;
    // move files and compute their contribution to the checksum
    // note it assumes no file has a length of zero
    while moving_file_id > 0 {
        let mut i = 0;
        while i < moving_file_id {
            if nums[moving_file_id * 2] <= nums[i * 2 + 1] - space_used[i] as i8 {
                let mut n = nums[moving_file_id * 2] as u8;
                while n > 0 {
                    n -= 1;
                    sum +=
                        (offsets[i] as u64 + nums[i * 2] as u64 + space_used[i] as u64 + n as u64)
                            * moving_file_id as u64;
                }
                space_used[i] += nums[moving_file_id * 2] as u8;
                // signal that the file has moved
                nums[moving_file_id * 2] = -nums[moving_file_id * 2];
                break;
            }
            i += 1;
        }
        moving_file_id -= 1;
    }

    // for un-moved files, compute their contribution to the checksum
    for i in 0..space_used.len() + 1 {
        let mut n = nums[i * 2];
        while n > 0 {
            n -= 1;
            sum += (offsets[i] as u64 + n as u64) * i as u64;
        }
    }

    println!("{}", sum);
    Ok(())
}

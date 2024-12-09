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
    let mut nums = Vec::<u8>::with_capacity(INPUT.len());
    for v in INPUT.as_bytes() {
        nums.push(*v - b'0');
    }

    let mut moving_file_id = nums.len() / 2;
    let mut reading_file_id = 0;
    let mut write_idx = 0;

    let mut sum = 0;
    while moving_file_id >= reading_file_id {
        // checksum for file blocks already in the right place
        while nums[reading_file_id * 2] > 0 {
            nums[reading_file_id * 2] -= 1;
            sum += write_idx * reading_file_id;
            write_idx += 1;
        }

        if moving_file_id <= reading_file_id {
            break;
        }

        // checksum for file parts as they move
        while nums[reading_file_id * 2 + 1] > 0 {
            nums[reading_file_id * 2 + 1] -= 1;
            nums[moving_file_id * 2] -= 1;
            sum += write_idx * moving_file_id;
            write_idx += 1;
            if nums[moving_file_id * 2] == 0 {
                moving_file_id -= 1;
            }
        }

        reading_file_id += 1;
    }

    println!("{}", sum);
    Ok(())
}

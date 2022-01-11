pub fn part_1(depths: &Vec<String>) -> u32 {
    let mut previous_value: Option<u32> = None;
    let mut increases = 0;

    // Loop through the depths
    for depth_str in depths {

      let depth = match depth_str.parse::<u32>() {
        Ok(depth) => depth,
        Err(_) => continue,
      };

      // If this is the first measurement, set the previous value
      if let None = previous_value {
        previous_value = Some(depth);
        continue;
      }

      // If the value is greater than the previous value, increase the number of increases
      if depth > previous_value.unwrap() {
        increases += 1;
      }
      previous_value = Some(depth);
    }
    increases
}

const WINDOW_SIZE: usize = 3;
pub fn part_2(depths: &Vec<String>) -> u32 {
    let mut increases = 0;
    for (i, _) in depths.iter().enumerate() {

        // if we're past the final two windows to compare
        if (i + WINDOW_SIZE + 1) > (depths.len() - 1) {
            break;
        }

        // Calculate the first window
        let mut window_a: u32 = 0;
        let start_idx = i;
        let end_idx = start_idx + WINDOW_SIZE;
        for j in start_idx..end_idx {
          match depths[j].parse::<u32>() {
            Ok(depth) => window_a += depth,
            Err(_) => panic!("Could not parse depth {}", depths[j]),
          };
        }

        // Calculate the second window
        let mut window_b: u32 = 0;
        let start_idx = i + 1;
        let end_idx = start_idx + WINDOW_SIZE;
        for j in start_idx..end_idx {
          match depths[j].parse::<u32>() {
            Ok(depth) => window_b += depth,
            Err(_) => panic!("Could not parse depth {}", depths[j]),
          };
        }

        // Record windowed depth increases
        if window_b > window_a {
            increases += 1;
        }
    }
    increases
}

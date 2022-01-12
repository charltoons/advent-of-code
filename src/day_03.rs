pub fn part_1(input: &Vec<String>) -> usize {

  // Get the number of bits, remove empty strings at beginning and end
  let bitcount = input[0].split("").collect::<Vec<&str>>().len() - 2;
  let mut accumulator = vec![0; bitcount];
  for number in input {
    for (i, bit) in number.split("").enumerate() {
      match bit {
        // Offset the accumulator to adjust for empty string at beginning
        "1" => accumulator[i - 1] += 1,
        // Do nothing for 0
        _ => continue,
      }
    }
  }
  let mut gamma = String::from("");
  let mut epsilon = String::from("");
  for acc in accumulator {
    if acc <= ((input.len() - 1) / 2) {
      gamma.push('0');
      epsilon.push('1');
    }
    else {
      gamma.push('1');
      epsilon.push('0');
    }
  }
  let gamma_int = usize::from_str_radix(&gamma, 2).unwrap();
  let epsilon_int = usize::from_str_radix(&epsilon, 2).unwrap();
  gamma_int * epsilon_int
}
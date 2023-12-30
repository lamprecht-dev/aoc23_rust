use anyhow::Result;

const AOC_DAY:i32 = 1;

fn main() -> Result<()> {
	let solution1 = solve(aoc::input(AOC_DAY), 1)?;
	let solution2 = solve(aoc::input(AOC_DAY), 2)?;
	println!("\n");
	println!("Solution 1: {}", solution1);
	println!("Solution 2: {}", solution2);
	println!("\n");

    Ok(())
}

fn solve(inp: String, part: i64) -> Result<i64> {
	Ok(inp
	.lines()
	.map(|line| {
		get_digit(line, true, part) * 10 + get_digit(line, false, part)
	})
	.sum())
}

fn get_digit(line: &str, first:bool, part: i64) -> i64{
	for i in 0..line.len(){
		let ii = if first{
			i
		}
		else{
			line.len() - i - 1
		};
		let c_int = line[ii..ii+1].parse::<i64>();
		if c_int.is_ok(){
			return c_int.unwrap()
		}
		if part == 2{
			let number_index = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
			for n in number_index {
				if line.len() >= ii + n.len() && line[ii..ii+n.len()] == *n{
					return number_index.iter().position(|&x| x == n).unwrap().try_into().unwrap();
				}
			}
		}
	}
	return 0
}

#[cfg(test)]
mod tests{
	use super::*;

    #[test]
    fn solve_works() -> Result<()> { 
		let s1 = solve(aoc::test(AOC_DAY, 1), 1)?;
		let s2 = solve(aoc::test(AOC_DAY, 2), 2)?;

		assert_eq!(s1, 142);
		assert_eq!(s2, 281);

		println!("\nTest Sucessful!");

        Ok(())
    }
}
use anyhow::Result;

const AOC_DAY:i32 = 7;

fn main() -> Result<()> {
	let (solution1, solution2) = solve(aoc::input(AOC_DAY))?;
	println!("\n");
	println!("Solution 1: {}", solution1);
	println!("Solution 2: {}", solution2);
	println!("\n");

    Ok(())
}

fn solve(inp: String) -> Result<(u64, u64)> {
    dbg!(inp);

	return Ok((0, 0));
}

#[cfg(test)]
mod tests{
	use super::*;

    #[test]
    fn solve_works() -> Result<()> { 
		let (s1, s2) = solve(aoc::test(AOC_DAY, 1))?;

		assert_eq!(s1, 0);
		assert_eq!(s2, 0);

		println!("\nTest Sucessful!");

        Ok(())
    }
}
use anyhow::Result;
use std::cmp;

const AOC_DAY:i32 = 2;

fn main() -> Result<()> {
	let (solution1, solution2) = (solve(aoc::input(AOC_DAY), false)?, solve(aoc::input(AOC_DAY), true)?);
	println!("\n");
	println!("Solution 1: {}", solution1);
	println!("Solution 2: {}", solution2);
	println!("\n");

    Ok(())
}

fn solve(inp: String, part: bool) -> Result<u64> {
	let max_games = inp.lines()
		.map(|line| {
			line
			.split(": ")
			.collect::<Vec<_>>()[1]
			.split("; ")
			.fold(vec![0, 0, 0], |mut acc: Vec<u64>, dline|{
				let dline_simple = parse_individual_draws(dline);
				for i in 0..acc.len(){
					acc[i] = cmp::max(acc[i], dline_simple[i]);
				}
				acc
			})
		});

	// Part 1
	if !part{
		return Ok(max_games.enumerate().filter_map(|(index, line)|{
			if line[0] > 12 || line[1] > 13 || line[2] > 14{
				return None;
			}
			Some((index + 1) as u64)
		}).sum());
	}

	// Part 2
	Ok(max_games.map(|line|{
		line.iter().product::<u64>()
	}).sum::<u64>())
}

fn parse_individual_draws(draw: &str) -> Vec<u64>{
	return draw.split(", ")
		.fold(vec![0, 0, 0], |mut acc: Vec<u64>, element|{
			let (amount, color) = element.split_once(" ").unwrap_or(("0", "none"));
			if color == "none"{
				return acc;
			}
			let acc_index = if color == "red" { 0 }
				else if color == "green" { 1 }
				else { 2 };
			
			acc[acc_index] = amount.parse::<u64>().unwrap_or(0);
			acc
		});
}

#[cfg(test)]
mod tests{
	use super::*;
    #[test]
    fn solve_works() -> Result<()> { 
		let s1 = solve(aoc::test(AOC_DAY, 1), false)?;
		let s2 = solve(aoc::test(AOC_DAY, 1), true)?;

		assert_eq!(s1, 8);
		assert_eq!(s2, 2286);

		println!("\nTest Sucessful!");

        Ok(())
    }
}
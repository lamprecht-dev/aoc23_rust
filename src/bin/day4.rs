use anyhow::Result;
use itertools::Itertools;

const AOC_DAY:i32 = 4;

fn main() -> Result<()> {
	let (solution1, solution2) = solve(aoc::input(AOC_DAY))?;
	println!("\n");
	println!("Solution 1: {}", solution1);
	println!("Solution 2: {}", solution2);
	println!("\n");

    Ok(())
}

fn solve(inp: String) -> Result<(i32, usize)> {
	let card_scores = inp.lines().map(calculate_scores).collect_vec();

	let final_score = card_scores.iter().map(|n| {
		return if *n == 0 { 0 } else { 2_i32.pow((n - 1) as u32) };
	}).sum::<i32>();

	let card_accu = card_scores.iter().enumerate().fold(vec![1; card_scores.len()], |mut acc:Vec<usize>, cur| {
		for i in 0..*cur.1{
			acc[cur.0 + i + 1] += acc[cur.0];
		}
		acc
	});

	return Ok((final_score, card_accu.iter().sum()));
}

fn calculate_scores(line: &str) -> usize{
	let (winning, selected) = line.split(": ").collect_vec()[1].split(" | ")
	.map( |r| r.split(" ")
		.map(|n| n.parse().unwrap_or(0)).collect_vec()
	).collect_tuple().unwrap();

	selected.iter().filter(|n| winning.contains(n) && **n != 0).count()
}

#[cfg(test)]
mod tests{
	use super::*;

    #[test]
    fn solve_works() -> Result<()> { 
		let (s1, s2) = solve(aoc::test(AOC_DAY, 1))?;

		assert_eq!(s1, 13);
		assert_eq!(s2, 30);

		println!("\nTest Sucessful!");

        Ok(())
    }
}
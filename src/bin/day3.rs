use anyhow::Result;
use itertools::Itertools;

const AOC_DAY:i32 = 3;

fn main() -> Result<()> {
	let (solution1, solution2) = solve(aoc::input(AOC_DAY))?;
	println!("\n");
	println!("Solution 1: {}", solution1);
	println!("Solution 2: {}", solution2);
	println!("\n");

    Ok(())
}

#[derive(Debug)]
struct Positions{
	x: i32,
	x2: Option<i32>,
	y: i32,
	value: Option<i32>,
	p_type: char
}

impl Positions{
	fn new(l: (usize, &str)) -> Vec<Self>{
		let (y, line) = l;
		let mut start_number:Option<usize> = None;	
		let mut current_value:u32 = 0;
		let mut positions:Vec<Positions> = Vec::new();
		for (x, c) in line.chars().enumerate(){
			if let Some(digit) = c.to_digit(10){
				current_value = current_value * 10 + digit;
				if start_number == None{
					start_number = Some(x);
				}
			}
			else {
				if start_number.is_some(){
					positions.push(Self{x: start_number.unwrap() as i32, x2: Some((x - 1) as i32), y: y as i32, value: Some(current_value as i32), p_type:  'n'});
					start_number = None;
					current_value = 0;
				}
				if c != '.'{
					positions.push(Self{x: x as i32, x2: None, y: y as i32, value: None, p_type: c});
				}
			}
		}
		if start_number.is_some(){
			positions.push(Self{x: start_number.unwrap() as i32, x2: Some((line.len() - 1) as i32), y: y as i32, value: Some(current_value as i32), p_type:  'n'});
		}
		return positions;
	}
}

fn solve(inp: String) -> Result<(i32, i32)> {
	let (gears, other, numbers) = inp
	.lines()
	.enumerate()
	.flat_map(|line| Positions::new(line))
	.fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, cur| {
		if cur.p_type == '*' {
			acc.0.push(cur);
			return acc;
		}
		if cur.p_type != 'n' {
			acc.1.push(cur);
			return acc;
		}
		acc.2.push(cur);
		acc
	});
	
	let adj_numbers = numbers.iter().filter(|n|{
		if gears.iter().any(|g| is_touching(n, g)){
			return true
		}
		other.iter().any(|o| is_touching(n, o))
	}).collect_vec();

	let ratios = gears.iter().filter_map(|g| {
		let touching_nums = adj_numbers.iter().filter(|adj_n| is_touching(adj_n, g)).collect_vec();
		if touching_nums.len() != 2 {
			return None
		}
		return Some(touching_nums[0].value.unwrap() * touching_nums[1].value.unwrap())

	}).collect_vec();

	return Ok((adj_numbers.iter().fold(0, |acc, cur| acc + cur.value.unwrap()), ratios.iter().sum()));
}

fn is_touching(n: &Positions, o: &Positions) -> bool{
	if o.y == n.y && (o.x == n.x2.unwrap() + 1 || o.x == n.x - 1){
		// Same row condition
		return true;
	}
	if (o.y == n.y + 1 || o.y == n.y - 1) && ((n.x-1)..(n.x2.unwrap()+2)).contains(&o.x){
		// Row above or below
		return true;
	}

	return false;
}

#[cfg(test)]
mod tests{
	use super::*;

    #[test]
    fn solve_works() -> Result<()> { 
		let (s1, s2) = solve(aoc::test(AOC_DAY, 1))?;

		assert_eq!(s1, 4361);
		assert_eq!(s2, 467835);

		println!("\nTest Sucessful!");

        Ok(())
    }
}
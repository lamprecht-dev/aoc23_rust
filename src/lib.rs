use std::fs;
use std::str::FromStr;
use anyhow::Result;

pub fn input(day: i32) -> String{
    fs::read_to_string(format!("./data/{}.input", day)).expect("Should have been able to read the file")
}

pub fn test(day: i32, nr: i32) -> String{
    fs::read_to_string(format!("./data/{}.{}.test", day, nr)).expect("Should have been able to read the file")
}

pub fn parse_lines<T>(str: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(str.lines()
    .filter_map(|s| 
        s.parse().ok())
    .collect())
}

pub fn parse_split<T>(str: &str, sep: &str) -> Result<Vec<T>>
where 
    T: FromStr,
{
    Ok(str.split(sep)
    .filter_map(|s| 
        s.parse().ok())
    .collect())
}

pub fn ints(str: &str) -> Result<Vec<i32>>{
    return parse_lines::<i32>(str);
}

pub fn parse_line_to_vec<T, F>(str: &str, f: F) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    F: Fn(&str) -> Option<Vec<T>>
{
    Ok(str.lines()
    .map(|line| f(line).unwrap())
    .collect())
} 

pub fn parse_char_to_vec<T, F>(str: &str, f: F) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    F: Fn(char) -> Option<T>
{
    Ok(str.lines()
    .map(|line| 
        line.chars().map(|c| f(c).unwrap()).collect()
    )
    .collect())
}  

// Thanks tjdeveries for the idea
pub fn neighbors(x: usize, y: usize, x_max: usize, y_max: usize, include_diagonal: bool) -> Vec<(usize, usize)>{
    let mut dirs = Vec::new();

    for dy in -1..2{
        for dx in -1..2{
            if dx == dy && dx == 0{
                continue;
            }

            let nx = match dx{
                1 => x.checked_add(1),
                -1 => x.checked_sub(1),
                _ => Some(x)
            };
            let ny = match dy{
                1 => y.checked_add(1),
                -1 => y.checked_sub(1),
                _ => Some(y)
            };

            if nx == None || ny == None || nx.unwrap() >= x_max || ny.unwrap() >= y_max{
                continue;
            }

            if !include_diagonal && dy != 0 && dx != 0{
                continue;
            }

            dirs.push((nx.unwrap(), ny.unwrap()))
        }
    }
    
    dirs
}
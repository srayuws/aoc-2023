use std::fs::read_to_string;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res, IResult,
};

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

enum Color {
    Red,
    Blue,
    Green,
}

struct Cubes {
    pub count: u32,
    pub color: Color,
}

fn parse_cube(input: &str) -> IResult<&str, Cubes> {
    let (input, count) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((
        map_res(tag("red"), |_| Ok::<Color, ()>(Color::Red)),
        map_res(tag("blue"), |_| Ok::<Color, ()>(Color::Blue)),
        map_res(tag("green"), |_| Ok::<Color, ()>(Color::Green)),
    ))(input)?;
    Ok((
        input,
        Cubes {
            count: count,
            color: color,
        },
    ))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (input, cubes) = nom::multi::separated_list1(tag(", "), parse_cube)(input)?;
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for cube in cubes {
        match cube.color {
            Color::Red => red += cube.count,
            Color::Blue => blue += cube.count,
            Color::Green => green += cube.count,
        }
    }
    Ok((
        input,
        Set {
            red: red,
            blue: blue,
            green: green,
        },
    ))
}

fn parse_sets(input: &str) -> IResult<&str, Vec<Set>> {
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = nom::multi::separated_list1(tag("; "), parse_set)(input)?;

    Ok((input, sets))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;

    let (input, sets) = parse_sets(input)?;

    Ok((input, Game { id: id, sets: sets }))
}

fn is_bad_game(game: &Game) -> bool {
    for set in &game.sets {
        if set.red > 12 {
            return true;
        }
        if set.green > 13 {
            return true;
        }
        if set.blue > 14 {
            return true;
        }
    }
    return false;
}

fn game_power(game: &Game) -> u32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for set in &game.sets {
        if red < set.red {
            red = set.red;
        }
        if green < set.green {
            green = set.green;
        }
        if blue < set.blue {
            blue = set.blue;
        }
    }
    return red*blue*green;
}

fn main() {
    let sample = "Game 1234: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let game = parse_game(sample).unwrap().1;
    println!("{:?}", game);

    let args = std::env::args().collect::<Vec<String>>();
    let input = args.get(1).expect("No input provided");
    let lines = read_to_string(input).expect("failed to read input file");
    let mut sum = 0;

    for line in lines.lines() {
        let game = parse_game(line).unwrap().1;
        sum += game_power(&game);
        
        if !is_bad_game(&game) {
            //sum += game.id;
            // println!("{:?}", game.id);
        }
    }
    print!("Sum of good games: {}", sum);
}

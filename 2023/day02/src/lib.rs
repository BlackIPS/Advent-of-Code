pub mod errors;
pub mod part1;
pub mod part2;


#[derive(Debug)]
pub struct Game {
    id: usize,
    reveals: Vec<Reveal>
}

impl Game {
    pub fn parse(input: &str) -> Result<Game, errors::AoCError> {
        let (game, rest) = input.split_once(": ").unwrap();
        
        Ok(Game {
            id: game.strip_prefix("Game ").unwrap().parse().unwrap(),
            reveals: rest
                .split("; ")
                .map(|reveal| {
                    let mut temp = Reveal {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    
                    for cube_info in reveal.split(", ") {
                        let (count, color) = cube_info.split_once(" ").unwrap();
                        let count: usize = count.parse().unwrap();
                        match color {
                            "red" => temp.red = count,
                            "green" => temp.green = count,
                            "blue" =>  temp.blue = count,
                            _ => panic!("Unbekannter Identifier")
                        }
                    }
                    
                    temp
                })
                .collect()
        })
    }
    
    pub fn is_compliant_to_rules(&self) -> bool {
        for reveal in &self.reveals {
            if reveal.red > 12 || reveal.green > 13 || reveal.blue > 14 {
                return false
            }
        }
        
        true
    }
    
    pub fn minimum_cubes_necessary(&self) -> Reveal {
        let mut result = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };
        
        for reveal in &self.reveals {
            if reveal.red > result.red { result.red = reveal.red }
            if reveal.green > result.green { result.green = reveal.green }
            if reveal.blue > result.blue { result.blue = reveal.blue }
        }
        
        result
    }
}

#[derive(Debug)]
pub struct Reveal {
    red: usize,
    green: usize,
    blue: usize
}

impl Reveal {
    pub fn power_of(&self) -> usize {
        self.red * self.green * self.blue
    }
}


#[tracing::instrument]
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}


#[cfg(debug_assertions)]
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();
}


#[cfg(not(debug_assertions))]
pub fn init_tracing() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();
}
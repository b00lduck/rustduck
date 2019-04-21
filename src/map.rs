extern crate rand;

use map::rand::Rng;

pub struct Map {
    state: [[i8; 64]; 64]
}

#[derive(PartialEq,Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Map {

    pub fn new() -> Map {
        let mut ret = Map {
            state: [[0;64]; 64]
        };  
        ret.create();
        return ret
    }

    pub fn create(&mut self) {
        println!("Now generating map");

        let posx = 32;
        let posy = 1;
        let direction = Direction::South;        

        self.step(posx, posy, &direction);

        self.print();
    }

    fn check_surroundings(&mut self, posx: i16, posy: i16, oldDirection: &Direction) -> bool {

        if posy < 1 || posx < 1 || posy > 62 || posx > 62 {
            return true;
        }

        let nx = posx as usize;
        let ny = posy as usize;

        if oldDirection != &Direction::South {
            if self.state[ny-1][nx] > 0 {
                println!("N blocked");
                return true
            }
        }
        if oldDirection != &Direction::North {
            if self.state[ny+1][nx] > 0 {
                println!("S blocked");
                return true
            }           
        }
        if oldDirection != &Direction::East {
            if self.state[ny][nx-1] > 0 {
                println!("W blocked");                
                return true
            }            
        }
        if oldDirection != &Direction::West {
            if self.state[ny][nx+1] > 0 {
                println!("E blocked");
                return true
            }
        }

        return false;
    }

    fn step(&mut self, posx: i16, posy: i16, direction: &Direction) {
        println!("{:?} {:?} {:?}", direction, posx, posy);

        if posy < 0 || posy >= 64 || posx < 0 || posx >= 64 {
            return
        }
       
        let new_direction = change_direction(posx, posy, direction);

        if self.check_surroundings(posx, posy, new_direction) {
            println!("Surroundings check negative");
            return
        }

        self.state[posy as usize][posx as usize] = 1;

        /*
        let mut rng = rand::thread_rng();
        let q = rng.gen_range(0, 40);

        match q {
            1 => {
                println!("4-Split");
                self.step(posx, posy - 1, Direction::North);
                self.step(posx, posy + 1, Direction::South);
                self.step(posx - 1, posy, Direction::West);
                self.step(posx + 1, posy, Direction::East);                
            },
            2 => {
                if direction != Direction::North && direction != Direction::South {
                    println!("2-Split NS");
                    self.step(posx, posy - 1, Direction::North);
                    self.step(posx, posy + 1, Direction::South);
                }
            },
            3 => {
                if direction != Direction::West && direction != Direction::East {
                    println!("2-Split WE");
                    self.step(posx - 1, posy, Direction::West);
                    self.step(posx + 1, posy, Direction::East);
                }
            },
            _ => {}
        } 
        */

        match direction {
            Direction::North => self.step(posx, posy - 1, direction),
            Direction::South => self.step(posx, posy + 1, direction),
            Direction::West => self.step(posx - 1, posy, direction),
            Direction::East => self.step(posx + 1, posy, direction),
        }

    }

    fn print(&self) {

    for a in self.state.iter() {
        for b in a.iter() {
            if *b == 0 {
                print!(".");
            } else {
                print!("O");
            }
        }
        print!("\n");
    }

    }

    

}


fn change_direction(posx: i16, posy: i16, direction: &Direction) -> &Direction {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0, 15);

    match n {
        1 => match direction {
            Direction::North => &Direction::West,
            Direction::South => &Direction::East,
            Direction::West => &Direction::North,
            Direction::East => &Direction::South
        }
        2 => match direction {
            Direction::North => &Direction::East,
            Direction::South => &Direction::West,
            Direction::West => &Direction::South,
            Direction::East => &Direction::North
        }
        _ => match direction {
            Direction::North => &Direction::North,
            Direction::South => &Direction::South,
            Direction::West => &Direction::West,
            Direction::East => &Direction::East
        }
    }
    
}
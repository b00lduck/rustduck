extern crate rand;

use map::generator::rand::Rng;

use map::direction::Direction;
use map::direction::Vector2;

const SIZEX: usize = 32;
const SIZEY: usize = 32;

pub struct Map {
    pub state: [[i8; SIZEX]; SIZEY],
    pub size: Vector2
}


impl Map {

    pub fn new() -> Map {
        let mut ret = Map {
            state: [[0;SIZEX]; SIZEY],
            size: Vector2 {x: SIZEX as i16, y: SIZEY as i16}
        };  
        // Start the recursion
        ret.step((SIZEX / 2) as i16, 0, &Direction::South);
        //ret.print();
        return ret
    }


    fn step(&mut self, posx: i16, posy: i16, direction: &Direction) {
        //println!("{:?} {:?} {:?}", direction, posx, posy);

        if posy < 0 || posy+1 >= SIZEY as i16 || posx < 0 || posx+1 >= SIZEX as i16 {
            return
        }
        
        let new_direction = change_direction(&self.state, posx, posy, direction);        

        
        let mut rng = rand::thread_rng();
        let q = rng.gen_range(0..40);

        match q {
            1 => {
                //println!("4-Split");
                self.step(posx, posy - 1, &Direction::North);
                self.step(posx, posy + 1, &Direction::South);
                self.step(posx - 1, posy, &Direction::West);
                self.step(posx + 1, posy, &Direction::East);                
            },
            2 => {
                if direction != &Direction::North && direction != &Direction::South {
                    //println!("2-Split NS");
                    self.step(posx, posy - 1, &Direction::North);
                    self.step(posx, posy + 1, &Direction::South);
                }
            },
            3 => {
                if direction != &Direction::West && direction != &Direction::East {
                    //println!("2-Split WE");
                    self.step(posx - 1, posy, &Direction::West);
                    self.step(posx + 1, posy, &Direction::East);
                }
            },
            _ => {}
        } 
        

        let v = new_direction.to_vector();
        if new_direction != Direction::Blocked {
            self.state[posy as usize][posx as usize] = 1;
            self.step(posx + v.x, posy + v.y, &new_direction);
        }

    }    

    /*
    fn print(&self) {

        for a in self.state.iter() {
            for b in a.iter() {
                if *b == 0 {
                    print!("@");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
    }
    */
}

fn get_possible_direction(state: &[[i8; SIZEX]; SIZEY], posx: i16, posy: i16, direction: &Direction) -> u8 {

    if check_newdir_free(state, posx, posy, &direction.turn_cw()) {
        return 2        
    }

    if check_newdir_free(state, posx, posy, &direction.turn_ccw()) {
        return 3        
    }
    
    if check_newdir_free(state, posx, posy, &direction.clone()) {
        return 1        
    }

    return 0
}

fn check_block_free(state: &[[i8; SIZEX]; SIZEY], checkx: i16, checky: i16) -> bool {
    
    if checkx >= SIZEX as i16 || checkx < 1 || checky >= SIZEY as i16 || checky < 1 {
        return false
    }
    //println!("Checking {:?},{:?}: {:?}", checkx, checky,state[checky as usize][checkx as usize]);
    if state[checky as usize][checkx as usize] != 0 {
        return false
    }
    return true
}

fn check_newdir_free(state: &[[i8; SIZEX]; SIZEY], posx: i16, posy: i16, direction: &Direction) -> bool {
    
    let check_dir = direction.to_vector();
    if !check_block_free(state, posx + check_dir.x * 2, posy + check_dir.y * 2) ||
       !check_block_free(state, posx + check_dir.x, posy + check_dir.y) {
        return false
    }

    let check_dir_left = direction.turn_ccw().to_vector();
    if !check_block_free(state, posx + check_dir.x + check_dir_left.x ,posy + check_dir.y + check_dir_left.y) {
        return false
    }

    let check_dir_right = direction.turn_cw().to_vector();
    if !check_block_free(state, posx + check_dir.x + check_dir_right.x, posy + check_dir.y + check_dir_right.y) {
        return false
    }

    return true
}

fn change_direction(state: &[[i8; SIZEX]; SIZEY], posx: i16, posy: i16, direction: &Direction) -> Direction {    
    let ret = change_direction_inner(state, posx, posy, direction);
    //if direction != &ret {
    //    println!(" from {:?} to {:?}", direction, ret);
    //}
    return ret
}

fn change_direction_inner(state: &[[i8; SIZEX]; SIZEY], posx: i16, posy: i16, direction: &Direction) -> Direction {

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..10);

    if n == 0 || !check_newdir_free(state, posx, posy, direction) {
        let q = get_possible_direction(state, posx, posy, direction);
        //println!("PDC: {:?}", q);
        match q {        
            1 => return direction.clone(),
            2 => return direction.turn_cw(),
            3 => return direction.turn_ccw(),
            _ => return Direction::Blocked
            
        };
    }

    return direction.clone();
  
    
}



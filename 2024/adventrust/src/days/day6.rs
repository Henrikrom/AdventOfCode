use std::io::Empty;
use std::thread;
use std::time::Duration;

pub struct Day6 {
}


#[derive(Clone)]
#[derive(Default)]
pub struct GuardMap {
    pub guard: GuardData,
    pub map: Vec<Vec<MapUnit>>
}

#[derive(Clone)]
#[derive(Default)]
pub struct GuardData {
    pub guard_pos: (i32,i32),
    pub guard_dir: (i32,i32),
}

#[derive(Clone)]
pub enum MapUnit {
    Empty(bool), //true if guard was here at one point
    Guard(GuardData),
    Obstruction,
    Edge
}

impl Day6 {
    pub fn parse_map(input: &String) -> GuardMap {
        let mut guard_map: GuardMap = GuardMap::default();
    
        let mut map: Vec<Vec<MapUnit>> = Vec::new();
        for line in input.lines() {
            let mut map_row: Vec<MapUnit> = Vec::new();
            for c in line.chars() {
                if c == 'X' {
                    map_row.push(MapUnit::Edge);
                }  
                else if c == '^' {
                    let guard_data: GuardData = GuardData { 
                        guard_pos: (0,0),
                        guard_dir: (-1,0)
                    };  
                    guard_map.guard = guard_data.clone(); 

                    map_row.push(MapUnit::Guard(guard_data));
                }
                else if c == '#' {
                    map_row.push(MapUnit::Obstruction);
                }
                else if c == '.' {
                    map_row.push(MapUnit::Empty(false));
                }
            }

            map.push(map_row);
        }


        guard_map.map = map;

        return guard_map;
    }

    pub fn part1(map: &mut GuardMap) {
        let mut map = Day6::set_guard_data(map);
        
        loop {
            //Day6::print_map(&map);
            //thread::sleep(Duration::from_millis(500));   
            let next_guard_location_y: usize = (map.guard.guard_pos.0 + map.guard.guard_dir.0).try_into().unwrap();
            let next_guard_location_x: usize = (map.guard.guard_pos.1 + map.guard.guard_dir.1).try_into().unwrap();
            match &mut map.map[next_guard_location_y][next_guard_location_x] {
                MapUnit::Empty(guard_was_here) => {
                    let old_guard_y: usize = map.guard.guard_pos.0.try_into().unwrap();
                    let old_guard_x: usize = map.guard.guard_pos.1.try_into().unwrap();
                    map.map[old_guard_y][old_guard_x] = MapUnit::Empty(true);
                    map.guard.guard_pos.0 = next_guard_location_y.try_into().unwrap();
                    map.guard.guard_pos.1 = next_guard_location_x.try_into().unwrap();
                    map.map[next_guard_location_y][next_guard_location_x] = MapUnit::Guard(map.guard.clone());
                },
                MapUnit::Guard(_guard_data) => println!("Should not go here?!"),
                MapUnit::Obstruction => {
                    if map.guard.guard_dir == (-1, 0) {
                        map.guard.guard_dir = (0, 1)
                    }
                    else if map.guard.guard_dir == (0,1) { 
                        map.guard.guard_dir = (1, 0)
                    }
                    else if map.guard.guard_dir == (1,0) { 
                        map.guard.guard_dir = (0, -1)
                    }
                    else if map.guard.guard_dir == (0, -1) { 
                        map.guard.guard_dir = (-1, 0)
                    }
                },
                MapUnit::Edge => {
                    break;
                }
            }
        }

        let count = map.map.into_iter().flat_map(|map_unit| map_unit).filter(|unit| {
            if let MapUnit::Empty(guard_was_here) = unit {
                return *guard_was_here
            }
            else {
                false
            }
        }).count();
        println!("Move count: {}", count + 1);
    }


    pub fn set_guard_data(guard_map: &mut GuardMap) -> GuardMap {
       for i in 0..guard_map.map.len() {
        for j in 0..guard_map.map[0].len() {
            if let MapUnit::Guard(_guard_data) = &guard_map.map[i][j] {
                let i_as_i32 = i.try_into().unwrap();
                let j_as_i32 = j.try_into().unwrap();
                guard_map.guard.guard_pos = (i_as_i32, j_as_i32);
                guard_map.guard = GuardData {
                    guard_dir: (-1,0),
                    guard_pos: (i_as_i32, j_as_i32)
                };

                return guard_map.clone();
            }
        }
       }

       return guard_map.clone();
    }

    pub fn part2(map: &mut GuardMap) {
        let mut map = Day6::set_guard_data(map);

        let mut count = 0;
        for i in 0..map.map.len() {
            for j in 0..map.map[0].len() {
                if Day6::is_loop(&mut map, j, i) {
                    count += 1;
                }
            }
        }

        println!("Obstacle placements: {}", count);
    }

    pub fn is_loop (guard_map: &mut GuardMap, x: usize, y: usize) -> bool {
        let mut map: GuardMap = guard_map.clone();

        if let MapUnit::Empty(_) = &map.map[y][x] {
            map.map[y][x] = MapUnit::Obstruction;
            //Day6::print_map(&map);
            let mut found_edge = false;
            for _ in 0..10000 {
                let next_guard_location_y: usize = (map.guard.guard_pos.0 + map.guard.guard_dir.0).try_into().unwrap();
                let next_guard_location_x: usize = (map.guard.guard_pos.1 + map.guard.guard_dir.1).try_into().unwrap();
                match &mut map.map[next_guard_location_y][next_guard_location_x] {
                    MapUnit::Empty(guard_was_here) => {
                        let old_guard_y: usize = map.guard.guard_pos.0.try_into().unwrap();
                        let old_guard_x: usize = map.guard.guard_pos.1.try_into().unwrap();
                        map.map[old_guard_y][old_guard_x] = MapUnit::Empty(true);
                        map.guard.guard_pos.0 = next_guard_location_y.try_into().unwrap();
                        map.guard.guard_pos.1 = next_guard_location_x.try_into().unwrap();
                        map.map[next_guard_location_y][next_guard_location_x] = MapUnit::Guard(map.guard.clone());
                    },
                    MapUnit::Guard(_guard_data) => println!("Should not go here?!"),
                    MapUnit::Obstruction => {
                        if map.guard.guard_dir == (-1, 0) {
                            map.guard.guard_dir = (0, 1)
                        }
                        else if map.guard.guard_dir == (0,1) { 
                            map.guard.guard_dir = (1, 0)
                        }
                        else if map.guard.guard_dir == (1,0) { 
                            map.guard.guard_dir = (0, -1)
                        }
                        else if map.guard.guard_dir == (0, -1) { 
                            map.guard.guard_dir = (-1, 0)
                        }
                    },
                    MapUnit::Edge => {
                        found_edge = true;
                        break;
                    }

            }
        }
        //if !found_edge {
            //Day6::print_map(&map);
        //}
        return !found_edge;
       }
       else {
        return false;
       } 
    }

    pub fn print_map(guard_map: &GuardMap) {
       for i in 0..guard_map.map.len() {
        for j in 0..guard_map.map[0].len() {
            match &guard_map.map[i][j] {
                MapUnit::Empty(guard_was_here) => {
                    if !*guard_was_here {
                        print!(".");
                    }
                    else {
                        print!("o");
                    }
                },
                MapUnit::Obstruction => print!("#"),
                MapUnit::Guard(_) => print!("^"),
                MapUnit::Edge => print!("X")
            }
        }
        
        println!("");
       }
    }

}
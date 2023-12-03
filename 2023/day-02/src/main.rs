use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let max_cubes = vec![12, 13, 14];

    let mut game_id = 1;
    let mut id_sum = 0;
    for line in fs::read_to_string("src/input").unwrap().lines() {
        let cloned_line = line.split(":").collect::<Vec<_>>();
        let game = cloned_line.get(1).unwrap();
        let sets = &game.split(";").collect::<Vec<_>>();
        let mut cube_possible_flag = true;
        for set in sets {
            let cubes = set.split(",");

            for cube in cubes {
                match cube.trim().split(" ").last().unwrap() {
                    "red" => {
                        if (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                            .parse::<u32>()
                            .unwrap()
                            > max_cubes[0]
                        {
                            cube_possible_flag = false;
                        }
                    }
                    "green" => {
                        if (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                            .parse::<u32>()
                            .unwrap()
                            > max_cubes[1]
                        {
                            cube_possible_flag = false;
                        }
                    }
                    "blue" => {
                        if (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                            .parse::<u32>()
                            .unwrap()
                            > max_cubes[2]
                        {
                            cube_possible_flag = false;
                        }
                    }
                    _ => println!("err"),
                }
            }
        }
        if cube_possible_flag {
            id_sum += game_id
        }

        game_id += 1;
    }

    println!("part 1: {}", id_sum)
}

fn part2() {
    let mut sum = 0;
    for line in fs::read_to_string("src/input").unwrap().lines() {
        let cloned_line = line.split(":").collect::<Vec<_>>();
        let game = cloned_line.get(1).unwrap();
        let sets = &game.split(";").collect::<Vec<_>>();
        let mut highest_red = 0;
        let mut highest_green = 0;
        let mut highest_blue = 0;
        for set in sets {
            let cubes = set.split(",");

            for cube in cubes {
                match cube.trim().split(" ").last().unwrap() {
                    "red" => {
                        let reds = (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                            .parse::<u32>()
                            .unwrap();

                        if reds > highest_red {
                            highest_red = reds;
                        }
                    }
                    "green" => {
                        let greens =
                            (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                                .parse::<u32>()
                                .unwrap();

                        if greens > highest_green {
                            highest_green = greens;
                        }
                    }
                    "blue" => {
                        let blues = (*(cube.trim().split(" ").collect::<Vec<_>>().get(0).unwrap()))
                            .parse::<u32>()
                            .unwrap();

                        if blues > highest_blue {
                            highest_blue = blues;
                        }
                    }
                    _ => println!("err"),
                }
            }
        }
        sum += highest_blue * highest_green * highest_red
    }

    println!("part 2: {}", sum)
}

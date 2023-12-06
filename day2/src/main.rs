use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("no input file found");

    let mut sum = 0;
    let mut new_sum = 0;

    for game in file.lines() {
        let game = &game.split(":").collect::<Vec<&str>>()[0..2];
        let game_title = game[0].trim();
        let game_info = game[1].trim();

        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;

        let mut valid_game = true;
        let rolls = game_info.split(";").collect::<Vec<&str>>();
        for roll in rolls {
            let results = roll.split(",").collect::<Vec<&str>>();
            for result in results {
                let result = result.trim();
                let roll = result.split(" ").collect::<Vec<&str>>();
                let number = roll[0].trim();
                let number = number.parse::<u32>().unwrap();
                let color = roll[1].trim();

                if color == "red" && number > max_r {
                    max_r = number;
                }

                if color == "green" && number > max_g {
                    max_g = number;
                }

                if color == "blue" && number > max_b {
                    max_b = number;
                }

                if color == "red" && number > 12 {
                    valid_game = false;
                }

                if color == "green" && number > 13 {
                    valid_game = false;
                }
                if color == "blue" && number > 14 {
                    valid_game = false;
                }
            }
        }

        println!("{}: {} {} {}", game_title, max_r, max_g, max_b);
        let product = max_r * max_g * max_b;
        new_sum += product;

        if valid_game {
            let title_info = game_title.split(" ").collect::<Vec<&str>>();
            let id = title_info[1].trim();
            let id = id.parse::<u32>().unwrap();
            sum += id;
        }
    }

    println!("sum: {}", sum);
    println!("new sum: {}", new_sum);
}

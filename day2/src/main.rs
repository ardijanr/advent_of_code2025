fn main() {
    parse(INPUT);
}

fn parse(text: &str) {
    let mut invalid_ids: Vec<u64> = Vec::new();
    for (left, right) in text.split(',').map(|txt| {
        let a = txt
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        (a[0], a[1])
    }) {
        for i in left..=right {
            let original_number = i.to_string();
            let mut numbers_to_check = original_number.to_string();
            let mut current_number = String::new();

            // Part 1
            // if number_str.to_string().len() % 2 == 0 {
            //     let half_length = number_str.len() / 2;
            //     if number_str[..half_length] == number_str[half_length..] {
            //         invalid_ids.push(number_str.parse::<u64>().unwrap());
            //         continue;
            //     }
            // }

            while !numbers_to_check.is_empty() {
                current_number.push(numbers_to_check.remove(0));

                if original_number.replace(&current_number, "").is_empty()
                    && current_number.len() < (original_number.len() / 2) + 1
                {
                    invalid_ids.push(original_number.to_string().parse::<u64>().unwrap());
                    break;
                }
            }
        }
    }

    // dbg!(&invalid_ids);

    dbg!(invalid_ids.iter().sum::<u64>());
}

// const INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
const INPUT: &'static str = "851786270-851907437,27-47,577-1044,1184-1872,28214317-28368250,47766-78575,17432-28112,2341-4099,28969-45843,5800356-5971672,6461919174-6461988558,653055-686893,76-117,2626223278-2626301305,54503501-54572133,990997-1015607,710615-802603,829001-953096,529504-621892,8645-12202,3273269-3402555,446265-471330,232-392,179532-201093,233310-439308,95134183-95359858,3232278502-3232401602,25116215-25199250,5489-8293,96654-135484,2-17";

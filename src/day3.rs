pub fn generator(input: &str) -> String {
    input.to_string()
}

pub fn part_1(input: &str) -> usize {
    let mut com = vec![];
    for sack in input.lines() {
        let (c1, c2) = sack.split_at(sack.len() / 2);

        let c1 = c1.chars().map(char_to_int).collect::<Vec<usize>>();
        let c2 = c2.chars().map(char_to_int).collect::<Vec<usize>>();


        // find a common item in both compartments
        let mut common = c1.iter().filter(|&x| c2.contains(x)).collect::<Vec<&usize>>();

        // remove duplicates from common
        common.sort();
        common.dedup();

        // println!("{:?}", common);
        com.extend(common);

    }


    let sum = com.iter().sum::<usize>();
    // println!("sum pt1: {}", sum);
    sum

}


pub fn part_2(input: &str) -> usize {
    let mut groups = vec![];

    // split the lines into groups of 3
    for i in (0..input.lines().count()).step_by(3) {
        let mut group = vec![];
        for j in 0..3 {
            group.push(input.lines().nth(i + j).unwrap());
        }
        groups.push(group);
    }

    // println!("{:#?}", groups);

    // for each group, find the common items between the 3 rucksacks

    let mut comgroups = vec![];
    for group in groups {
        let mut common = vec![];

        // Get the char that are in all 3 rucksacks
        for i in 0..group[0].len() {
            let c = group[0].chars().nth(i).unwrap();
            if group[1].contains(c) && group[2].contains(c) {
                common.push(char_to_int(c));
            }
        }

        // remove duplicates from common
        common.sort();
        common.dedup();

        // println!("{:?}", common);
        comgroups.extend(common);
    }
    // println!("{:?}", comgroups);
    let sum2 = comgroups.iter().sum::<usize>();
    // println!("sum pt2: {}", sum2);
    sum2

}



fn char_to_int(c: char) -> usize {
    // convert alphabet to integer
    // using some unicode magic
    // a-z = 1-26
    // A-Z = 27-52
    if c.is_lowercase() {
        c as usize - 96
    } else {
        c as usize - 64 + 26
    }
}




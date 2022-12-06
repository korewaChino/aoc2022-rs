pub fn generator(input: &str) -> Vec<u32> {
    let pairs: [u32; 2] = input.lines().map(|assign| {
        let mut splits = assign
            .splitn(4, &[',', '-'])
            .map(|s| s.parse::<u8>().unwrap());
        let start1 = splits.next().unwrap();
        let end1 = splits.next().unwrap();
        let start2 = splits.next().unwrap();
        let end2 = splits.next().unwrap();

        let left = start1 <= start2 && end1 >= end2;
        let right = start2 <= start1 && end2 >= end1;
        let covered = left || right;

        let overlap = start1 <= end2 && start2 <= end1;

        (covered, overlap)
    }).fold([0, 0], |[acc_c, acc_o], (covered, overlap)| {
        let acc_c = if covered { acc_c + 1 } else { acc_c };
        let acc_o = if overlap { acc_o + 1 } else { acc_o };
        [acc_c, acc_o]
    });
    pairs.to_vec()
}



pub fn part_1(input: &Vec<u32>) -> usize {
    input[0] as usize
}

pub fn part_2(input: &Vec<u32>) -> usize {
    input[1] as usize
}

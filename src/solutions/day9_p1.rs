pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let mut history = vec![];

        history.push(
            line.split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );

        while !history.last().unwrap().iter().all(|x| *x == 0) {
            let last = history.last().unwrap();
            let mut next = vec![];

            for i in 0..last.len() - 1 {
                next.push(last[i + 1] - last[i]);
            }

            history.push(next);
        }

        sum += history.iter().map(|step| step.last().unwrap()).sum::<i32>();
    }

    sum.to_string()
}

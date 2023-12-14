use std::collections::HashMap;

struct ArrangementCount<'a> {
    conditions: &'a str,
    counts: Vec<usize>,
    cache: HashMap<(usize, usize), usize>,
}

impl<'a> ArrangementCount<'a> {
    fn new(conditions: &'a str, counts: Vec<usize>) -> Self {
        Self {
            conditions,
            counts,
            cache: HashMap::new(),
        }
    }

    fn find_arrangements(&mut self, mut condition_idx: usize, mut count_idx: usize) -> usize {
        // terminal condition 1
        if count_idx >= self.counts.len() {
            let result = match self.conditions[condition_idx..]
                .chars()
                .all(|x| x == '.' || x == '?')
            {
                true => 1,
                false => 0,
            };

            return result;
        }

        // skip '.'s
        while let Some(ch) = self.conditions.chars().nth(condition_idx) {
            if ch != '.' {
                break;
            }
            condition_idx += 1;
        }

        // check cache
        if let Some(v) = self.cache.get(&(condition_idx, count_idx)) {
            return *v;
        }

        let cache_count_idx = count_idx;

        // terminal condition 2
        if condition_idx >= self.conditions.len() {
            return 0;
        }

        // terminal condition 3
        if self.conditions.len() - condition_idx
            < self.counts[count_idx..].iter().sum::<usize>()
                + self.counts[count_idx..].iter().count()
                - 1
        {
            return 0;
        }

        let mut result = 0;

        if self.conditions.chars().nth(condition_idx).unwrap() == '?' {
            result += self.find_arrangements(condition_idx + 1, count_idx);
        }

        let cur_count = self.counts[count_idx];
        count_idx += 1;

        let mut cur_section = "#".repeat(cur_count);

        if count_idx < self.counts.len() {
            cur_section += ".";
        }

        if condition_idx + cur_section.len() <= self.conditions.len()
            && self.conditions[condition_idx..condition_idx + cur_section.len()]
                .chars()
                .zip(cur_section.chars())
                .all(|(c, s)| c == '?' || c == s)
        {
            result += self.find_arrangements(condition_idx + cur_section.len(), count_idx);
        }

        self.cache.insert((condition_idx, cache_count_idx), result);

        result
    }
}

pub fn solution(lines: Vec<&str>) -> String {
    let mut sum = 0;

    for line in lines {
        let (conditions, counts) = line.split_once(' ').unwrap();
        let counts = counts
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut arrangement_count = ArrangementCount::new(conditions, counts);

        sum += arrangement_count.find_arrangements(0, 0);
    }

    sum.to_string()
}

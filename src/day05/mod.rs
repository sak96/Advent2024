use fxhash::FxHashMap;
use fxhash::FxHashSet;
use std::cmp::Ordering;
use std::io::BufRead;

fn is_valid(page_order: &FxHashMap<u64, Vec<u64>>, update: &[u64]) -> bool {
    let mut invalid_pages = FxHashSet::<u64>::default();
    for page in update {
        if invalid_pages.contains(page) {
            return false;
        }
        if let Some(pages) = page_order.get(page) {
            invalid_pages.extend(pages);
        }
    }
    true
}

pub fn run<'a>(reader: Box<dyn BufRead + 'a>) {
    let mut page_order = FxHashMap::default();
    let mut lines = reader.lines().map_while(Result::ok);

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut split = line.split('|');
        let pre = split.next().unwrap().parse().unwrap();
        let post = split.next().unwrap().parse().unwrap();
        page_order
            .entry(post)
            .and_modify(|v: &mut Vec<_>| v.push(pre))
            .or_insert(vec![pre]);
    }

    let updates: Vec<Vec<u64>> = lines
        .map(|line| line.split(',').map(|i| i.parse().unwrap()).collect())
        .collect();

    let mut valid_updates = 0;
    let mut fix = 0;
    for mut update in updates {
        if is_valid(&page_order, &update) {
            valid_updates += update.get((update.len()) / 2).unwrap();
        } else {
            update.sort_by(|a, b| {
                page_order
                    .get(a)
                    .and_then(|v| v.contains(b).then_some(Ordering::Greater))
                    .unwrap_or(Ordering::Less)
            });
            fix += update.get((update.len()) / 2).unwrap();
        }
    }
    println!("{valid_updates}");
    println!("{fix}");
}

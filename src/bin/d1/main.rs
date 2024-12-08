const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct TuplesWithDistance {
    tuple: (i32, i32),
    distance: i32,
}

#[derive(Debug)]
struct TuplesWithDistanceSimilarity {
    tuple: (i32, i32),
    distance: i32,
    similarity: i32,
}

fn main() {
    let input = INPUT;
    let splitted = input.split("\n").collect::<Vec<_>>();
    let mut list_a = splitted
        .iter()
        .map(|v| v.split_once("   ").unwrap().0)
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    list_a.sort();
    let mut list_b = splitted
        .iter()
        .map(|v| v.split_once("   ").unwrap().1)
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    list_b.sort();
    let tuples_with_distance: Vec<TuplesWithDistance> = list_a
        .into_iter()
        .zip(list_b)
        .map(|v| TuplesWithDistance {
            tuple: v,
            distance: (v.0 - v.1).abs(),
        })
        .collect::<Vec<_>>();
    let tuples_with_similarity = tuples_with_distance
        .iter()
        .map(|v| {
            let a: i32 = tuples_with_distance
                .iter()
                .filter(|c| c.tuple.1 == v.tuple.0)
                .collect::<Vec<_>>()
                .len()
                .try_into()
                .unwrap();
            let res = TuplesWithDistanceSimilarity {
                distance: v.distance,
                tuple: v.tuple,
                similarity: a * v.tuple.0,
            };
            res
        })
        .collect::<Vec<_>>();
    let total_distance = tuples_with_distance
        .iter()
        .fold(0, |acc, x| acc + x.distance);
    let total_similarity = tuples_with_similarity
        .iter()
        .fold(0, |acc, x| acc + x.similarity);
    dbg!(total_distance);
    dbg!(total_similarity);
}

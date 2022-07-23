// Topic: Lifetimes & Structures
//
// Requirements:
// * Display just the names and titles of persons from the mock-data.csv file
// * The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

struct Names<'a> {
    inner: Vec<&'a str>,
}

struct Titles<'a> {
    inner: Vec<&'a str>,
}

fn parse_csv_rows(data: &str) -> Vec<&str> {
    data.split("\n").skip(1).collect()
}

fn main() {
    let data = parse_csv_rows(MOCK_DATA);
    let names: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(1))
        .collect();
    let names = Names { inner: names };

    let titles: Vec<_> = data
        .iter()
        .filter_map(|line| line.split(',').nth(4))
        .collect();
    let titles = Titles { inner: titles };

    let data = names.inner.iter().zip(titles.inner.iter());

    for d in data.take(10) {
        println!("Name: {:?}, title: {:?}", d.0, d.1)
    }
}

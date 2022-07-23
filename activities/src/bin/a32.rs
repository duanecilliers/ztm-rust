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

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

fn parse_csv_rows(data: &str) -> Vec<&str> {
    data.split("\n").collect()
}

fn parse_csv_cols(rows: Vec<&str>) -> Vec<Vec<&str>> {
    rows.iter().map(|row| row.split(",").collect()).collect()
}

fn collect_people(data: Vec<Vec<&str>>) -> Vec<Person> {
    data.iter()
        .map(|row| Person {
            name: row[1],
            title: row[4],
        })
        .collect()
}

fn main() {
    // println!("{:?}", MOCK_DATA);
    // println!("{:?}", parse_csv_rows(MOCK_DATA));
    let rows = parse_csv_rows(MOCK_DATA);
    // println!("{:?}", parse_csv_cols(rows));
    let data = parse_csv_cols(rows);
    println!("{:?}", data);
    let people = collect_people(data);
    for person in people.iter() {
        println!("Person: {:?}", person);
    }
}

// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Pressure(u16);

#[derive(Debug, PartialEq)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn message_match(tile: Tile) {
    use Tile::*;
    match tile {
        // Brick(style) if style == BrickStyle::Gray || style == BrickStyle::Red => {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {:?}", brick)
        }
        Brick(style) => println!("{:?} brick", style),
        Water(pressure) if pressure >= Pressure(10) => println!("High water pressure!"),
        Water(pressure) => println!("Water pressure level: {:?}", pressure),
        Grass | Dirt | Sand => println!("Ground tile"),
        // Treasure(chest) if (chest.amount >= 100 && chest.content == TreasureItem::Gold) => {
        Treasure(TreasureChest { amount, content })
            if amount >= 100 && content == TreasureItem::Gold =>
        {
            println!("Lots of gold!")
        }
        _ => (),
    }
}

fn main() {
    let red_brick = Tile::Brick(BrickStyle::Red);
    message_match(red_brick);
    let dungeon_brick = Tile::Brick(BrickStyle::Dungeon);
    message_match(dungeon_brick);
    let high_pressure_water = Tile::Water(Pressure(20));
    message_match(high_pressure_water);
    let dirt = Tile::Dirt;
    message_match(dirt);
    let sand = Tile::Sand;
    message_match(sand);
    let precious_gold = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 101,
    });
    message_match(precious_gold);
}

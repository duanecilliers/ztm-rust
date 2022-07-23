// Topic: Trait Objects
//
// Summary:
//   A contractor wants a program that can sum the cost of materials based
//   on how many square meters are required for a job.
//
// Requirements:
// * Calculate multiple material types with different costs
// * Must be able to process a list of varying materials
// * Material types and cost includes:
//   * Carpet - $10 per square meter
//   * Tile - $15 per square meter
//   * Wood - $20 per square meter
// * Square meters must be taken into account
//
// Notes:
// * Create a trait that can be used to retrieve the cost of a material
// * Create trait objects and store them in a vector for processing
// * Use a function to calculate the total cost
// * Process at least 3 different materials

trait Material {
    fn get_price(&self, meters: f64) -> f64;
}

struct Carpet {
    price: f64,
}
impl Material for Carpet {
    fn get_price(&self, meters: f64) -> f64 {
        self.price * meters
    }
}

struct Tile {
    price: f64,
}
impl Material for Tile {
    fn get_price(&self, meters: f64) -> f64 {
        self.price * meters
    }
}

struct Wood {
    price: f64,
}
impl Material for Wood {
    fn get_price(&self, meters: f64) -> f64 {
        self.price * meters
    }
}

fn get_total_cost(materials: Vec<Box<dyn Material>>, meters: f64) -> f64 {
    let mut total = 0.0;
    for material in materials {
        total += material.get_price(meters)
    }
    total
}

fn main() {
    let carpet = Box::new(Carpet { price: 10.0 });
    let tile = Box::new(Tile { price: 15.0 });
    let wood = Box::new(Wood { price: 15.0 });
    let materials: Vec<Box<dyn Material>> = vec![carpet, tile, wood];
    println!("Total cost: {:?}", get_total_cost(materials, 20.0))
}

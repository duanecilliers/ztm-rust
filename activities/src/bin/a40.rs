// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * Corporate must be able to access the rentals at a storefront
// * Storefronts must be able to rent out vehicles
// * Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum Vehicle {
    Car,
    Truck,
}

#[derive(Debug, Hash, PartialEq, PartialOrd)]
enum Status {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct Rental {
    vin: String,
    vehicle: Vehicle,
    status: Status,
}

type Rentals = Rc<RefCell<Vec<Rental>>>;

#[derive(Debug)]
struct Corporate(Rentals);

#[derive(Debug)]
struct StoreFront(Rentals);

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_status() {
        let vehicles = vec![
            Rental {
                vin: "123".to_owned(),
                status: Status::Available,
                vehicle: Vehicle::Car,
            },
            Rental {
                vin: "234".to_owned(),
                status: Status::Maintenance,
                vehicle: Vehicle::Truck,
            },
        ];

        let vehicles = Rc::new(RefCell::new(vehicles));

        let corporate = Corporate(Rc::clone(&vehicles));
        let storefront = StoreFront(Rc::clone(&vehicles));

        {
            let mut rentals = storefront.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Available);
                car.status = Status::Rented;
            }
        }

        {
            let mut rentals = corporate.0.borrow_mut();
            if let Some(car) = rentals.get_mut(0) {
                assert_eq!(car.status, Status::Rented);
                car.status = Status::Available;
            }
        }

        let rentals = storefront.0.borrow();
        if let Some(car) = rentals.get(0) {
            assert_eq!(car.status, Status::Available);
        }
    }
}

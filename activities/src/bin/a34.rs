// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
struct Luggage<Status> {
    id: f32,
    status: Status,
}
impl<Status> Luggage<Status> {
    fn transition<NextStatus>(self, status: NextStatus) -> Luggage<NextStatus> {
        Luggage {
            id: self.id,
            status: status,
        }
    }
}

impl Luggage<CheckIn> {
    fn new(id: f32) -> Self {
        Self {
            id,
            status: CheckIn,
        }
    }
    fn checkin(self) -> Luggage<OnLoading> {
        self.transition(OnLoading)
    }
}

impl Luggage<OnLoading> {
    fn load(self) -> Luggage<OffLoading> {
        self.transition(OffLoading)
    }
}

impl Luggage<OffLoading> {
    fn pickup(self) -> Luggage<AwaitingPickup> {
        self.transition(AwaitingPickup)
    }
}

impl Luggage<AwaitingPickup> {
    fn end_custody(self) -> Luggage<EndCustody> {
        self.transition(EndCustody)
    }
}

#[derive(Debug)]
struct CheckIn;

#[derive(Debug)]
struct OnLoading;

#[derive(Debug)]
struct OffLoading;

#[derive(Debug)]
struct AwaitingPickup;

#[derive(Debug)]
struct EndCustody;

fn main() {
    let processed_luggage = Luggage::new(1.0).checkin().load().pickup().end_custody();
    println!("Luggage: {:?}", processed_luggage)
}

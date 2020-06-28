use sir::sir::person::PersonState;
use sir::sir::world::{World, PopulationDistribution};
use sir::sir::virus::Virus;

#[test]
fn infect_closeby_users() {
    // We make a grid with 1 user infected, next step should be 5 -> 13 -> 25
    let mut virus = Virus::corona();
    // each step, infect all neighbours
    virus.distance = 12.0;
    let expected_infected = [1, 5, 13, 25];
    virus.infection_rate = 1.0;
    let mut world = World::new(100, 100.0, 100.0, virus, PopulationDistribution::Grid);
    world.config(0.0);
    for index in 0..4 {
        let mut count = 0;
        for person in world.people() {
            if let PersonState::Infectious(_) = person.get_state() {
                count += 1;
            }
        }
        assert_eq!(expected_infected[index], count);
        world.update();
    }
}
/// The type implementing this trait should not hold any information, it is just a workaround for dynamically dispatching the correct methods
/// If you want to keep state, create another struct and instantiate it in the solution
pub trait Solution<T> {
    const DAY: usize;
    fn get_day(&self) -> usize {
        Self::DAY
    }
    fn part1(&self, input: &str) -> Option<T>;
    fn part2(&self, input: &str) -> Option<T>;
    fn part3(&self, input: &str) -> Option<T>;
}

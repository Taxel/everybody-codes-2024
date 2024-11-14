pub trait Solution<T> {
    const DAY: usize;
    fn get_day(&self) -> usize {
        Self::DAY
    }
    fn part1(&mut self, input: &str) -> Option<T>;
    fn part2(&mut self, input: &str) -> Option<T>;
    fn part3(&mut self, input: &str) -> Option<T>;
}

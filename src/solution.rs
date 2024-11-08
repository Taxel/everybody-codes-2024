pub trait Solution<T> {
    const DAY: usize;
    fn get_day(&self) -> usize {
        Self::DAY
    }
    fn part1(&self, input: &str) -> Option<T>;
    fn part2(&self, input: &str) -> Option<T>;
    fn part3(&self, input: &str) -> Option<T>;
}

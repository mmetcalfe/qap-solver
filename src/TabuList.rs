extern crate rand;
use self::rand::Rng;
use std::cmp;

#[derive(Debug, Clone)]
pub struct TabuList {
    pub tabu_times : Vec<Vec<usize>>,
    pub tabu_tenure : usize,
}

impl TabuList {
    pub fn new(size : usize) -> TabuList {
        TabuList {
            // tabu_times: vec!(0;(size*(size-1))/2)
            tabu_times: vec!(vec!(0; size); size),
            tabu_tenure: 2*size,
        }
    }
    pub fn get_entry(&self, swap : (usize, usize)) -> usize {
        let a = cmp::min(swap.0, swap.1);
        let b = cmp::max(swap.0, swap.1);
        self.tabu_times[a][b]
    }

    pub fn set_entry(&mut self, swap : (usize, usize), until : usize) {
        let a = cmp::min(swap.0, swap.1);
        let b = cmp::max(swap.0, swap.1);
        self.tabu_times[a][b] = until
    }

    pub fn is_tabu(&self, swap : (usize, usize), iteration : usize) -> bool {
        self.get_entry(swap) > iteration
    }

    pub fn make_tabu(&mut self, swap : (usize, usize), until : usize) {
        self.set_entry(swap, until)
    }

    pub fn halve_all_counts(&mut self, iteration : usize) {
        for i in (0..self.tabu_times.len()) {
            for j in (0..i) {
                let val = self.get_entry((i,j));
                let diff = if val < iteration {iteration} else {val - iteration};
                self.set_entry((i, j), iteration + diff / 2)
            }
        }
    }

    pub fn subtract_from_all_counts(&mut self, diff : usize) {
        for i in (0..self.tabu_times.len()) {
            for j in (0..i) {
                let val = self.get_entry((i,j));
                if diff < val {
                    self.set_entry((i, j), val - diff);
                } else {
                    self.set_entry((i, j), 0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabu() {
        let mut tabu_lst = TabuList::new(4);
        let iteration = 0;
        assert!(!tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));

        tabu_lst.make_tabu((1,2), 5);
        let iteration = 0;
        assert!(tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));
        let iteration = 2;
        assert!(tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));
        let iteration = 4;
        assert!(tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));

        let iteration = 5;
        assert!(!tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));

        let iteration = 50;
        assert!(!tabu_lst.is_tabu((1,2), iteration));
        assert!(!tabu_lst.is_tabu((3,0), iteration));
    }

    #[test]
    fn test_reverse() {
        let mut tabu_lst = TabuList::new(4);
        tabu_lst.make_tabu((1,2), 5);
        let iteration = 1;
        assert!(tabu_lst.is_tabu((1,2), iteration));
        assert!(tabu_lst.is_tabu((2,1), iteration));
    }

    #[test]
    fn test_halve_all() {
        let mut tabu_lst = TabuList::new(4);
        tabu_lst.make_tabu((0,2), 5);
        tabu_lst.make_tabu((1,2), 7);
        tabu_lst.make_tabu((3,1), 12);
        tabu_lst.make_tabu((0,3), 50);

        let iteration = 5;
        tabu_lst.halve_all_counts(iteration);
        println!("{}", tabu_lst.get_entry((0,2)));
        assert!(tabu_lst.get_entry((0,2)) == 5);
        println!("{}", tabu_lst.get_entry((1,2)));
        assert!(tabu_lst.get_entry((1,2)) == 6);
        println!("{}", tabu_lst.get_entry((3,1)));
        assert!(tabu_lst.get_entry((3,1)) == 8);
        println!("{}", tabu_lst.get_entry((0,3)));
        assert!(tabu_lst.get_entry((0,3)) == 27);
    }

    #[test]
    fn test_subtract_from_all() {
        let mut tabu_lst = TabuList::new(4);
        tabu_lst.make_tabu((0,1), 2);
        tabu_lst.make_tabu((0,2), 5);
        tabu_lst.make_tabu((1,2), 7);
        tabu_lst.make_tabu((3,1), 12);
        tabu_lst.make_tabu((0,3), 50);

        let iteration = 5;
        tabu_lst.subtract_from_all_counts(3);
        println!("{}", tabu_lst.get_entry((0,1)));
        assert!(tabu_lst.get_entry((0,1)) == 0);
        println!("{}", tabu_lst.get_entry((0,2)));
        assert!(tabu_lst.get_entry((0,2)) == 2);
        println!("{}", tabu_lst.get_entry((1,2)));
        assert!(tabu_lst.get_entry((1,2)) == 4);
        println!("{}", tabu_lst.get_entry((3,1)));
        assert!(tabu_lst.get_entry((3,1)) == 9);
        println!("{}", tabu_lst.get_entry((0,3)));
        assert!(tabu_lst.get_entry((0,3)) == 47);
    }
}

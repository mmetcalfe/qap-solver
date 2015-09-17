extern crate rand;
use self::rand::Rng;
// use std::num;

#[derive(Debug, Clone)]
pub struct Permutation {
    pub image : Vec<u32>,
}

// TODO: Make the element type generic.
impl Permutation {
    pub fn identity(size : usize) -> Permutation {
        Permutation { image: (0..size as u32).collect() }
    }

    pub fn from_image(image : Vec<u32>) -> Permutation {
        Permutation { image: image }
    }

    pub fn random(size : usize) -> Permutation {
        // let mut image : Vec<u32> = vec!(0; size);
        let mut rng = rand::thread_rng();
        // for x in image.iter_mut() {
        //     *x = rng.gen_range(0, size as u32);
        // }

        let mut image : Vec<u32> = (0..size as u32).collect();
        rng.shuffle(&mut image[..]);
        Permutation { image: image }
    }

    pub fn num_transpositions(size : usize) -> usize {
        size*(size-1) / 2
    }

    pub fn indexed_transposition(size : usize, index : usize) -> Permutation {
        if index >= Permutation::num_transpositions(size) {
            panic!("indexed_transposition: invalid index {} for size {}", index, size);
        }

        let (i, j) = triangle_index(index);

        let mut perm = Permutation::identity(size);

        perm.image[i] = j as u32;
        perm.image[j] = i as u32;

        perm
    }

    pub fn random_transposition(size : usize) -> Permutation {
        let index = rand::thread_rng().gen_range(0, Permutation::num_transpositions(size));
        Permutation::indexed_transposition(size, index)
    }

    pub fn compose(&self, other : &Permutation) -> Permutation {
        let mut vec = self.image.clone();

        for i in 0..vec.len() {
            vec[i] = self.image[(other.image[i]) as usize]
        }

        Permutation::from_image(vec)
    }
}

pub fn triangle_index(index : usize) -> (usize, usize) {
    /*
    n(n-1)/2 options:
      c
    r +
      1 +
      2 3 +
      4 5 6 +
      7 8 9 d +

      let k = index
      k ~= r(r+1)/2
      => r^2 + r - 2*k = 0
      => r = (-1 +-sqrt(2-4*1*(-2k)))/2
      => r = floor(-0.5 + sqrt(2+8k)/2)
    */

    let k = index as f32;
    let r = (-0.5 + (2.0+8.0*k).sqrt()/2.0).floor() as usize;
    let c = index - (r*(r+1)) / 2;
    (r+1, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_index() {
        println!("{:?}", (0..10).map(triangle_index).collect::<Vec<_>>());
        assert_eq!(triangle_index(0), (1, 0));
        assert_eq!(triangle_index(4), (3, 1));
        assert_eq!(triangle_index(8), (4, 2));
    }

    #[test]
    fn compose_with_identity() {
        let id = Permutation::identity(5);
        let p1 = Permutation::from_image(vec!(1, 0, 2, 4, 3));
        let r1 = id.compose(&p1);
        println!("r1: {:?}", r1.image);
        println!("p1: {:?}", p1.image);
        assert!(r1.image == p1.image);
    }

    #[test]
    fn compose_wikipedia() {
        let p1 = Permutation::from_image(vec!(4, 3, 2, 1, 0));
        let p2 = Permutation::from_image(vec!(1, 3, 0, 2, 4));
        let r1 = p1.compose(&p2);
        println!("r1: {:?}", r1.image);
        println!("p1: {:?}", p1.image);
        println!("p2: {:?}", p2.image);
        assert!(r1.image == vec!(3, 1, 4, 2, 0));
    }

    #[test]
    fn compose_wikipedia_reverse() {
        let p1 = Permutation::from_image(vec!(1, 3, 0, 2, 4));
        let p2 = Permutation::from_image(vec!(4, 3, 2, 1, 0));
        let r1 = p1.compose(&p2);
        println!("r1: {:?}", r1.image);
        println!("p1: {:?}", p1.image);
        println!("p2: {:?}", p2.image);
        assert!(r1.image == vec!(4, 2, 0, 3, 1));
    }

    #[test]
    fn compose_with_permutation() {
        let p1 = Permutation::from_image(vec!(3, 2, 4, 0, 1));
        let p2 = Permutation::from_image(vec!(1, 0, 2, 4, 3));
        let r1 = p1.compose(&p2);
        println!("r1: {:?}", r1.image);
        println!("p2: {:?}", p2.image);
        println!("p1: {:?}", p1.image);
        assert!(r1.image == vec!(2, 3, 4, 1, 0));
    }
}

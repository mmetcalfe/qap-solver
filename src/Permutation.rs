pub struct Permutation {
    image : Vec<u32>,
}

// TODO: Make the element type generic.
impl Permutation {
    pub fn from_image(image : Vec<u32>) -> Permutation {
        Permutation { image: image }
    }

    pub fn compose(&self, other : &Permutation) -> Permutation {
        let mut vec = self.image.clone();

        for i in 0..vec.len() {
            vec[i] = self.image[(other.image[i]) as usize]
        }

        Permutation::from_image(vec)
    }
}

#[test]
fn compose_with_identity() {
    let id = Permutation::from_image(vec!(0, 1, 2, 3, 4));
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

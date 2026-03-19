mod counter;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    brand: String,
}

fn filter_shoes_of_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn filter_shoes_of_size_test() {
        let shoes = vec![Shoe {size: 2, brand: String::from("A")}, Shoe {size: 4, brand: String::from("B")}, 
        Shoe {size: 6, brand: String::from("C")}, Shoe {size: 8, brand: String::from("D")}];

        print!("Shoes: {:?}", shoes);

        assert!(filter_shoes_of_size(shoes, 2)[0].size <= 2);
    }

    #[test]
    fn test_iter() {
    let mut v1 = vec![1,2,3];
    
    let mut v1_iter = v1.iter_mut();

    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v2 = vec![1,2,3,4];
        let sum: i32 = v2.iter().sum();

        assert_eq!(sum, 10);
    }


    #[test]
    fn iterator_collect() {
        let v2 = vec![1,2,3,4];
        
        let v3: Vec<i32> = v2.iter().map(|x| x + 1).collect();

        assert_eq!(v3, vec![2,3,4,5]);
    }


}

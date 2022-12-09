mod newtypeiterator;

#[cfg(test)]
mod tests {

    use super::newtypeiterator::{NewType};

    #[test]
    fn it_works(){

        let mut x = NewType(vec![1, 2, 3, 4, 5]);

        let mut collection: Vec<&i32> = Vec::new();

        for item in x.iter_mut() {
            collection.push(item);
        }

        assert_eq!(vec![&mut 1, &mut 2, &mut 3, &mut 4, &mut 5], collection);

    }
    
}
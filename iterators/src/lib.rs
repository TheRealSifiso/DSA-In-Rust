mod newtypeiterator;

#[cfg(test)]
mod tests {

    use super::newtypeiterator::{NewType};

    #[test]
    fn it_works(){

        let x = NewType(vec![1, 2, 3, 4, 5]);

        let mut collection: Vec<&i32> = Vec::new();

        for item in x.iter() {
            collection.push(item);
        }

        assert_eq!(vec![&1, &2, &3, &4, &5], collection);

    }
    
}
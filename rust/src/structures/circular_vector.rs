/// definition
#[derive(Debug)]
pub struct CircularVector<T> {
    vec: Vec<T>,
    index: usize,
}

/// implementations
impl<T> CircularVector<T> {

    /// constructs empty struct
    pub fn new() -> Self {
        CircularVector {
            vec: Vec::new(),
            index: 0,
        }
    }

    /// constructs struct from existing vector with the first element being the current element
    pub fn from(vec: Vec<T>) -> Self {
        CircularVector { 
            vec, 
            index: 0,
        }
    }

    ///tries to get an element from the vector, if it can't will error.
    pub fn get(&self) -> Result<&T, &'static str> {
        match self.is_empty() {
            true => Err("Tried to index past vector length!")?,
            false => Ok(&self.vec[self.index]),
        }
    }

    pub fn remove(&mut self) -> Result<T, &'static str> {
        match self.is_empty() {
            true => Err("Tried to remove past vector length!")?,
            false => Ok(self.vec.remove(self.index)),
        }
    }

    /// inserts element after current element
    pub fn insert_after(&mut self, element: T)  {
        if self.is_empty() {
            self.vec.insert(self.index, element);
        }
        else {
            self.vec.insert(self.index + 1, element);
        }
    }

    /// inserts element before current element
    pub fn insert_before(&mut self, element: T)  {
        self.vec.insert(self.index, element);
    }

    /// rotates the vector clockwise
    pub fn rotate(&mut self) {
        self.index = (self.index + 1) % self.vec.len()
    }

    /// rotates the vector counter-clockwise
    pub fn rotate_back(&mut self) {
        //needed as negative values are unsupported.
        if self.index == 0 {
            self.index = self.size()
        }
        self.index = self.index - 1 % self.size()
    }

    /// returns whether the list is empty or not
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    /// returns size of the vector
    pub fn size(&self) -> usize {
        self.vec.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        let vec = vec![1,2,5,4,2];
        let cv: CircularVector<i32> = CircularVector::from(vec);

        assert_eq!(cv.vec, vec![1,2,5,4,2]);
    }

    #[test]
    fn get() {
        let vec = vec![1,2,5,4,2];
        let cv: CircularVector<i32> = CircularVector::from(vec);

        assert_eq!(cv.get().unwrap(), &1);
    }

    #[test]
    fn insert_after_and_rotate() {
        let vec = vec![1,2,5,4,2];
        let mut cv: CircularVector<i32> = CircularVector::from(vec);
        
        cv.insert_after(10);
        cv.rotate();

        assert_eq!(cv.get().unwrap(), &10);
    }

    #[test]
    fn insert_before_and_rotate_back() {
        let vec = vec![1,2,5,4,2];
        let mut cv: CircularVector<i32> = CircularVector::from(vec);
        
        cv.insert_after(10);
        for i in 0..(cv.size() - 1) {
            cv.rotate_back();
        }

        assert_eq!(cv.get().unwrap(), &10);
    }

    #[test]
    fn is_empty() {
        let cv: CircularVector<i32> = CircularVector::new();

        assert!(cv.is_empty());
    }

    #[test]
    fn size() {
        let vec = vec![1,4,12,7];
        let cv = CircularVector::from(vec);
        assert_eq!(cv.size(), 4);
    }

}
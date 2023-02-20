use std::collections::HashMap;


/// used as a type for each point
pub type Point = (i32, i32, i32);
pub type Step = (i32, i32, i32);

//--------------------------------------------------------------------------------------

/// Struct representing the board
pub struct Board<T> {
    x_size: i32,
    y_size: i32,
    z_size: i32,
    map: HashMap<Point, T>, //holds point, player pairs
}

impl<T> Board<T> {
    /// constructor for just supplying bounds
    pub fn new(x_size: i32, y_size: i32, z_size: i32) -> Result<Self, &'static str> {

        if x_size < 1 || y_size < 1 || z_size < 1 {
            Err("bounds must be greater than 0.")?
        }

        Ok(Self {
            x_size,
            y_size,
            z_size,
            map: HashMap::new(),
        })
    }


    /// will check to see if the provided point is inside bounds
    fn in_bounds(&self, point: &Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.2 >= 0 && point.0 < self.x_size && point.1 < self.y_size && point.2 < self.z_size
    }

    /// inserts into the board at the point with the value
    pub fn insert(&mut self, point: Point, val: T) -> Result<(), &'static str>{
        if !self.in_bounds(&point) {
            Err("Out of bounds.")?
        }
        self.map.insert(point, val);
        Ok(())
    }

    /// gets a value from the board at the point
    pub fn get_point(&self, point: &Point) -> Result<Option<&T>, &'static str> {
        if !self.in_bounds(point) {
            Err("out of bounds.")?
        }
        Ok(self.map.get(&point))
    }

    /// returns a list of points defined by the line definition supplied
    pub fn get_line(&self, point: &Point, step: &Step) -> Result<Vec<Option<&T>>, &'static str> {
        //check initial point
        if !self.in_bounds(point) {
            Err("out of bounds.")?
        }

        let mut point = point.to_owned();
        let mut list = Vec::new();

        loop {
            match self.get_point(&point) {
                Ok(it) => list.push(it),
                Err(_) => {
                    break Ok(list);
                }
            }
            point = (point.0 + step.0, point.1 + step.1, point.2 + step.2);
        }       
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_good_input() {
        let board: Board<i32> = Board::new(1,1,1).unwrap();

        assert!(board.x_size == 1 && board.y_size == 1 && board.z_size == 1);
    }

    #[test]
    #[should_panic]
    fn new_bad_input() {
        let board: Board<i32> = Board::new(0,0,0).unwrap();
    }

    #[test]
    fn in_bounds_good_input() {
        let board: Board<i32> = Board::new(1,1,1).unwrap();
        assert!(board.in_bounds(&(0,0,0)));
    }

    #[test]
    fn in_bounds_bad_input() {
        let board: Board<i32> = Board::new(1,1,1).unwrap();
        assert!(!board.in_bounds(&(1,1,1)));
    }

    #[test]
    fn insert_good_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((3,4,0), 0).unwrap();
        //just should not panic
    }

    #[test]
    #[should_panic]
    fn insert_bad_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((6,-1,0), 10).unwrap();
    }

    #[test]
    fn get_point_good_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((3,4,0), 1).unwrap();
        assert_eq!(board.get_point(&(3,4,0)).unwrap().expect(""), &1);
    }

    #[test]
    #[should_panic]
    fn get_point_bad_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((6,-1,0), 10).unwrap();
        board.get_point(&(-1,4,0)).unwrap().expect("");
    }

    #[test]
    fn get_line_good_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((3,4,0), 1).unwrap();
        board.insert((3,4,1), 2).unwrap();
        let originalOptionVals = board.get_line(&(3,4,0), &(0,0,1)).unwrap();
        let mut originalVals = Vec::new();
        for option in originalOptionVals {
            match option {
                Some(it) => originalVals.push(it),
                None => {},
            };
        }
        let mut newVals = Vec::new();
        newVals.push(&1);
        newVals.push(&2);

        assert_eq!(originalVals, newVals);
    }

    #[test]
    #[should_panic]
    fn get_line_bad_input() {
        let mut board: Board<i32> = Board::new(5,5,5).unwrap();
        board.insert((6,-1,0), 10).unwrap();
        board.get_point(&(-1,4,0)).unwrap().expect("");
    }
}
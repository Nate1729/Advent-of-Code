pub mod helpers {
    #[derive(Clone)]
    pub struct Location {
        x: i32,
        y: i32,
    }

    impl Location {
        pub fn new() -> Self {
            Location { x: 0, y: 0 }
        }

        pub fn move_by_char(&mut self, c: char) {
            match c {
                '>' => self.x += 1,
                '<' => self.x -= 1,
                '^' => self.y += 1,
                'v' => self.y -= 1,
                _ => (),
            }
        }
    }

    impl PartialEq for Location {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    impl Eq for Location {}

    fn contains<P>(list: &Vec<P>, new: &P) -> bool
    where
        P: Eq,
    {
        for element in list {
            if element == new {
                return true;
            }
        }
        false
    }

    pub fn add_location(location_vec: &mut Vec<Location>, new_location: Location) -> () {
        if !contains(location_vec, &new_location) {
            location_vec.push(new_location);
        }
    }
}

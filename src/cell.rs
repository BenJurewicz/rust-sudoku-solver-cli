use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct UncollapsedCell {
    possible_values: HashSet<u8>
}

impl UncollapsedCell {
    fn new() -> Self {
        UncollapsedCell {
            possible_values: HashSet::from([1,2,3,4,5,6,7,8,9])
        }
    }

    fn get_entropy(&self) -> u8 {
        self.possible_values.len() as u8
    }

    fn remove(&mut self, value: u8) -> Result<(), ()> {
        self.possible_values.remove(&value);
        if self.possible_values.is_empty() {
            Err(())
        } else {
            Ok(())
        }
    }

    fn collapse(self) -> Cell {
        Cell::Collapsed(self.possible_values.into_iter().next().unwrap())
    }
}

#[derive(Debug, Clone)]
pub enum Cell {
    Collapsed(u8),
    Uncollapsed(UncollapsedCell)
}

impl Cell {
    pub fn new_empty() -> Self {
        Cell::Uncollapsed(UncollapsedCell::new())
    }

    pub fn new_filled(value: u8) -> Self {
        Cell::Collapsed(value)
    }

    pub fn get_entropy(&self) -> u8{
        match self {
            Cell::Uncollapsed(c) => c.get_entropy(),
            Cell::Collapsed(_) => panic!("get_entropy called on a collapsed cell"),
        }
    }

    /// If the cell has more than one possible value, the function returns
    /// a copy of the uncollapsed cell with that value removed. <br>
    /// Otherwise, it returns the state of the cell before collapsing.
    pub fn collapse(&mut self) -> Cell{
        let mut cell = self.clone();
        *self = match self {
            Cell::Uncollapsed(c) => c.clone().collapse(),
            Cell::Collapsed(_) => panic!("collapse called on a collapsed cell"),
        };
        if cell.get_entropy() > 1 {
            let Cell::Collapsed(value) = *self else { unreachable!() };
            cell.remove(value).unwrap();
        }
        cell
    }

    pub fn remove(&mut self, value: u8) -> Result<(), ()> {
        match self {
            Cell::Uncollapsed(c) => c.remove(value),
            Cell::Collapsed(v) => if *v != value { Ok(()) } else { Err(()) }
        }
    }

}

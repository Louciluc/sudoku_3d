
#[derive(Clone, PartialEq, Eq)]
pub struct Cell {
    val: Option<usize>,
    needs_update: bool,
    open_options: Vec<usize>,
}

impl Cell {
    pub fn new(val_: Option<usize>) -> Cell {
        return Cell{val:val_, needs_update:true, open_options: vec![]};
    }
    pub fn update_value(&mut self, new_val: Option<usize>) {
        self.val = new_val;
        self.needs_update = true;
    }
    pub fn update_open_options(&mut self, new_options: Vec<usize>) {
        self.open_options = new_options;
        self.needs_update = false;
    }
    pub fn value (&self) -> Option<usize>{return self.val;}
    pub fn needs_update (&self) -> bool {return self.needs_update;}
    pub fn open_options (&self) -> &Vec<usize> {return &self.open_options;}
}
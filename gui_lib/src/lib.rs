pub struct AveragedCollection {
    list: Vec<i32>,
    sum: i32,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.sum += value;
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        match self.list.pop() {
            Some(res) => {
                self.sum -= 1;
                self.update_average();
                Some(res)
            },
            None => None
        }
    }

    fn update_average(&mut self) {
        self.average = self.sum as f64 / self.list.len() as f64;
    }
}
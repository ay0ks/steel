#[derive(Clone, Debug, Default)]
pub struct PeekBuffer<T>
where
    T: Clone,
{
    data: Vec<T>,
    history: Vec<T>,
    index: usize,
    history_index: usize,
}

impl<T> PeekBuffer<T>
where
    T: Clone,
{
    pub fn new(data: Vec<T>) -> Self {
        Self {
            data,
            history: Vec::new(),
            index: 0,
            history_index: 0,
        }
    }

    pub fn peek_at(&self, index: usize) -> Option<&T> {
        if index >= self.data.len() {
            None
        } else {
            self.data.get(self.index)
        }
    }

    pub fn peek_from_to(
        &self,
        range_begin: usize,
        range_end: usize,
        step: usize,
    ) -> Option<Vec<&T>> {
        if range_end > self.data.len() {
            None
        } else {
            let mut result = Vec::new();
            for i in range_begin..range_end {
                result.push(self.data.get(i).unwrap());
            }
            Some(result)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.peek_at(self.index)
    }

    pub fn peek_many(&self, n: usize) -> Option<Vec<&T>> {
        if self.index + n > self.data.len() {
            None
        } else {
            self.peek_from_to(self.index, self.index + n, 1)
        }
    }

    pub fn peek_many_stepping(&self, n: usize, step: usize) -> Option<Vec<&T>> {
        if self.index + n * step > self.data.len() {
            None
        } else {
            self.peek_from_to(self.index, self.index + n * step, step)
        }
    }

    pub fn peek_behind(&self) -> Option<&T> {
        if self.index == 0 {
            None
        } else {
            self.peek_at(self.index - 1)
        }
    }

    pub fn peek_behind_many(&self, n: usize) -> Option<Vec<&T>> {
        if self.index < n {
            None
        } else {
            self.peek_from_to(self.index - n, self.index, 1)
        }
    }

    pub fn peek_behind_many_stepping(&self, n: usize, step: usize) -> Option<Vec<&T>> {
        if self.index < n * step {
            None
        } else {
            self.peek_from_to(self.index - n * step, self.index, step)
        }
    }

    pub fn advance(&mut self) {
        if self.index < self.data.len() + 1 {
            self.index += 1;
        }
    }

    pub fn advance_by(&mut self, n: usize) {
        if self.index + n < self.data.len() {
            self.index += n;
        }
    }

    pub fn advance_by_stepping(&mut self, n: usize, step: usize) {
        if self.index + n * step < self.data.len() {
            self.index += n * step;
        }
    }

    pub fn advance_to(&mut self, index: usize) {
        if index < self.data.len() {
            self.index = index;
        }
    }

    pub fn advance_to_end(&mut self) {
        self.index = self.data.len();
    }

    pub fn rewind(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn rewind_by(&mut self, n: usize) {
        if self.index >= n {
            self.index -= n;
        }
    }

    pub fn rewind_by_stepping(&mut self, n: usize, step: usize) {
        if self.index >= n * step {
            self.index -= n * step;
        }
    }

    pub fn rewind_to(&mut self, index: usize) {
        if index < self.index {
            self.index = index;
        }
    }

    pub fn rewind_to_beginning(&mut self) {
        self.index = 0;
    }

    pub fn eat(&mut self) -> Option<T> {
        if self.index < self.data.len() {
            let element = self.data.remove(self.index);
            self.history.push(element.clone());
            Some(element)
        } else {
            None
        }
    }

    pub fn eat_many(&mut self, n: usize) -> Option<Vec<T>> {
        if self.index + n > self.data.len() {
            None
        } else {
            let mut result = Vec::new();
            for _ in 0..n {
                result.push(self.data.remove(self.index));
            }
            self.history.extend(result.clone());
            Some(result)
        }
    }

    pub fn eat_many_stepping(&mut self, n: usize, step: usize) -> Option<Vec<T>> {
        if self.index + n * step > self.data.len() {
            None
        } else {
            let mut result = Vec::new();
            for i in 0..n {
                result.push(self.data.remove(self.index + i * step));
            }
            self.history.extend(result.clone());
            Some(result)
        }
    }

    pub fn restore(&mut self) -> Option<T> {
        if self.history.is_empty() {
            None
        } else {
            let element = self.history.pop().unwrap();
            self.data.insert(self.index, element.clone());
            Some(element)
        }
    }

    pub fn restore_many(&mut self, n: usize) -> Option<Vec<T>> {
        if self.history.len() < n {
            None
        } else {
            let mut result = Vec::new();
            for _ in 0..n {
                result.push(self.history.pop().unwrap());
            }
            for i in 0..n {
                self.data.insert(self.index + i, result[i].clone());
            }
            Some(result)
        }
    }

    pub fn restore_many_stepping(&mut self, n: usize, step: usize) -> Option<Vec<T>> {
        if self.history.len() < n {
            None
        } else {
            let mut result = Vec::new();
            for i in 0..n {
                result.push(self.history.pop().unwrap());
            }
            for i in 0..n {
                self.data.insert(self.index + i * step, result[i].clone());
            }
            Some(result)
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.history.clear();
    }
}

impl<T> Iterator for PeekBuffer<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            Some(self.eat().unwrap())
        } else {
            None
        }
    }
}

impl From<String> for PeekBuffer<char> {
    fn from(s: String) -> Self {
        Self::new(s.chars().collect())
    }
}

impl From<&str> for PeekBuffer<char> {
    fn from(s: &str) -> Self {
        Self::new(s.chars().collect())
    }
}

impl<T> FromIterator<T> for PeekBuffer<T>
where
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl<T> Into<Vec<T>> for PeekBuffer<T>
where
    T: Clone,
{
    fn into(self) -> Vec<T> {
        self.data
    }
}

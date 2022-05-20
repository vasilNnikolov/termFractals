pub struct Buffer<T> where 
    T: Copy
{
    pub size: (u16, u16), // width, height,
    pointers: (u16, u16), // col pointer and row pointer
    contents: Vec<Vec<T>>,
}

impl<T> Buffer<T> where
    T: Copy 
{
    pub fn new(size: (u16, u16), starting_item: T) -> Buffer<T> {
        let mut contents: Vec<Vec<T>> = Vec::new();
        for _ in 0..size.1 {
            let mut row: Vec<T> = Vec::new();
            for _ in 0..size.0 {
                row.push(starting_item);
            }
            contents.push(row);
        }
        Buffer {
            size: size, 
            pointers: (0, 0), 
            contents: contents,
        }
    }
    pub fn get(&self, x: u16, y: u16) -> Result<T, &'static str> {
        if x < self.size.0 && y < self.size.1 {
            let new_x = (x + self.pointers.0) % self.size.0;
            let new_y = (y + self.pointers.1) % self.size.1;
            return Ok(*self.contents.get(new_y as usize).unwrap().get(new_x as usize).unwrap());
        }
        Err("x and y should be in the size of the buffer")
    }
    fn put_item(&mut self, item: T, x_buffer: u16, y_buffer: u16) -> Result<(), &'static str> { // puts an item directly into the buffer
        if x_buffer >= self.size.0 || y_buffer >= self.size.1 {
            return Err("cannot put item in coordinate not in buffer");
        } 
        let row = &mut self.contents[y_buffer as usize];
        let cell = &mut row[x_buffer as usize];
        *cell = item;
        Ok(())
    }
    pub fn shift(&mut self, direction: Direction, times: u16, placeholder_item: T) -> Result<(), &'static str> {
        // if a pointer jumps over a row or column, it needs to be filled with placeholders 
        match direction {
            Direction::Down => {
                for _i in 0..std::cmp::min(times, self.size.1) {
                    if self.pointers.1 > 0 {
                        self.pointers.1 -= 1;
                    } else {self.pointers.1 = self.size.1 - 1;} 
                    for x in 0..self.size.0 {
                        self.put_item(placeholder_item, x, self.pointers.1)?;
                    }
                } 
            }, 
            Direction::Up => {
                for _i in 0..std::cmp::min(times, self.size.1) {
                    for x in 0..self.size.0 {
                        self.put_item(placeholder_item, x, self.pointers.1)?;
                    }
                    if self.pointers.1 < self.size.1 - 1 {
                        self.pointers.1 += 1;
                    } else {self.pointers.1 = 0;} 
                } 
            }
            Direction::Right => {
                for _i in 0..std::cmp::min(times, self.size.0) {
                    if self.pointers.0 > 0 {
                        self.pointers.0 -= 1;
                    } else {self.pointers.0 = self.size.0 - 1;} 
                    for y in 0..self.size.1 {
                        self.put_item(placeholder_item, self.pointers.0, y)?;
                    }
                } 
            }, 
            Direction::Left => {
                for _i in 0..std::cmp::min(times, self.size.0) {
                    for y in 0..self.size.1 {
                        self.put_item(placeholder_item, self.pointers.0, y)?;
                    }
                    if self.pointers.0 < self.size.0 - 1 {
                        self.pointers.0 += 1;
                    } else {self.pointers.0 = 0;} 
                } 
            }
        }
        Ok(())
    }
    pub fn put(&mut self, item: T, x_screen: u16, y_screen: u16) -> Result<(), &'static str> { // puts an item directly into the buffer
        if x_screen >= self.size.0 || y_screen >= self.size.1 {
            return Err("cannot put item in coordinate not in buffer");
        } 
        let row = &mut self.contents[((y_screen + self.pointers.1) % self.size.1) as usize];
        let cell = &mut row[((x_screen + self.pointers.0) % self.size.0) as usize];
        *cell = item;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Left, Right, Up, Down
}



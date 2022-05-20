pub struct Buffer<T> where 
    T: Copy
{
    pub size: (u16, u16) // width, height,
    pointers: (u16, u16) // row pointer and column pointer
    contents: Vec<Vec<T>>
}

impl<T> Buffer<T> {
    pub fn new(size: (u16, u16), contents: Vec<Vec<T>>) -> Result<Buffer<T>, &'static str> {
        // size is width, height
        if contents.len() != size.1 as usize {
            return Err("contents must be with shape (width, height)")
        }
        for row in contents.iter() {
            if row.len() != size.0 as usize {
                return Err("contents must be with shape (width, height)")
            }
        }
        Ok(Buffer {
            size: size, 
            pointers: (0, 0), 
            contents: contents,
        })
    }
    pub fn get(&self, x: u16, y: u16) -> Result<T, &'static str> {
        if x < self.size.0 && y < self.size.1 {
            let new_x = (x + self.pointers.0)%self.size.0;
            let new_y = (y + self.pointers.1)%self.size.1;
            return Ok(self.contents.get(new_y)?.get(new_x?));
        }
        Err("x and y should be in the size of the buffer")
    }
    pub fn shift(&mut self, direction: Direction, times: u16) {
        match direction {
            Direction::Down {
                self.pointers.1 -= times;
            }, 
            Direction::Up {
                self.pointers.1 += times;
            }
            Direction::Left {
                self.pointers.0 += times;
            }, 
            Direction::Right {
                self.pointers.0 -= times;
            }
        }
    }
}

pub struct Direction {
    Left, Right, Up, Down
}



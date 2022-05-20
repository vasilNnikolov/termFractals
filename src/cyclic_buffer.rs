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

    }
    pub fn shift(&mut self, direction: Direction, times: u16) {
        match direction {
            Direction::Down {

            }
        }
    }
}

pub struct Direction {
    Left, Right, Up, Down
}



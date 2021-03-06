use crate::terminal::{
    screen, 
    cyclic_buffer
};

pub fn render_status_bar(screen: &mut screen::Screen, n_iter: u16) -> Result<(), &'static str> {
    // should indicate the current scale, position, fps eventually
    // stat bar is a box in the top left
    let scale_str = String::from(format!("Scale (log10): {}", -screen.scale.log10()));
    let position_string = String::from(format!("Position: {:.7} + i*{:.7}", screen.center.re, screen.center.im));
    let iteration_string = String::from(format!("Number of iterations: {}", n_iter));

    let strings_to_render = vec![
        scale_str, 
        position_string,
        iteration_string, 
    ];

    let max_width = strings_to_render.iter().map(|string: &String| string.len()).max().unwrap();
    let height = strings_to_render.len();
    // horisontal bars
    for x in 1..max_width + 1 {
        screen.putchar(x as u16, 0, cyclic_buffer::Pixel::StatBar('-'))?;
        screen.putchar(x as u16, (height + 1) as u16, cyclic_buffer::Pixel::StatBar('-'))?;
    }
    // vertical bars
    for y in 1..height + 1 {
        screen.putchar(0, y as u16, cyclic_buffer::Pixel::StatBar('|'))?;
        screen.putchar((max_width + 1) as u16, y as u16, cyclic_buffer::Pixel::StatBar('|'))?;
    }
    // rows of text
    for index in 0..strings_to_render.len() {
        for x in 0..strings_to_render[index].len() {
            screen.putchar((x + 1) as u16, (index + 1) as u16, cyclic_buffer::Pixel::StatBar(strings_to_render[index].chars().nth(x).unwrap()))?;
        }
    }
    
    Ok(())
}

pub fn clear_stat_bar(screen: &mut screen::Screen) -> Result<(), &'static str> {
    let (w, h) = screen.buffer.size;
    for x in 0..w {
        for y in 0..h {
            if let cyclic_buffer::Pixel::StatBar(_) = screen.buffer.get(x, y)? {
                screen.buffer.put(cyclic_buffer::Pixel::Recompute, x, y)?;
            }
        }
    }
    Ok(())
}

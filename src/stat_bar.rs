use crate::term_io;

pub fn render_status_bar(screen: &mut term_io::Screen, fps: u16) -> Result<(), &'static str> {
    // should indicate the current scale, position, fps eventually
    // stat bar is a box in the top left
    let scale_str = String::from(format!("Scale (log10): {}", -screen.scale.log10()));
    let position_string = String::from(format!("Position: {:.7} + i*{:.7}", screen.center.re, screen.center.im));

    let strings_to_render = vec![
        scale_str, 
        position_string,
    ];

    let max_width = strings_to_render.iter().map(|string: &String| string.len()).max().unwrap();
    for x in 0..max_width {
        screen.putchar(x as u16, 0, term_io::Pixel::StatBar('-'))?;
    }
    for x in 0..max_width {
        screen.putchar(x as u16, (strings_to_render.len() + 2) as u16, term_io::Pixel::StatBar('-'))?;
    }

    Ok(())
}

pub fn clear_stat_bar(screen: &mut term_io::Screen) -> Result<(), &'static str> {
    let (w, h) = screen.buffer.size;
    for x in 0..w {
        for y in 0..h {
            if let term_io::Pixel::StatBar(_) = screen.buffer.get(x, y)? {
                screen.buffer.put(term_io::Pixel::Recompute, x, y)?;
            }
        }
    }
    Ok(())
}
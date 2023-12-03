use colored::*;

// color highlight part of the line
pub fn debug_color_line(line: &str, first_occurrence: (usize, usize), last_occurrence: (usize, usize)) -> String {
    let (f_start, f_end) = first_occurrence;
    let (l_start, l_end) = last_occurrence;

    let mut colored_line = String::new();
    colored_line.push_str(&line[..f_start]);

    // if same range, color range green
    if f_start == l_start && f_end == l_end {
        colored_line.push_str(&line[f_start..f_end].green().to_string());
    } 
    // if overlapping ranges, color overlapping range blue and rest of ranges red
    else if f_end > l_start {
        colored_line.push_str(&line[f_start..l_start].red().to_string());
        colored_line.push_str(&line[l_start..f_end].blue().to_string());
        colored_line.push_str(&line[f_end..l_end].red().to_string());
    }
    // the default case, color both ranges red
    else {
        colored_line.push_str( &line[f_start..f_end].red().to_string());
        colored_line.push_str(&line[f_end..l_start]);
        colored_line.push_str(&line[l_start..l_end].red().to_string());
    }
    colored_line.push_str(&line[l_end..]);
    colored_line
}
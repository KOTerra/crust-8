pub(crate) fn sinval(t: f32) -> f32 {
    t.sin()
}
pub(crate) fn fill_matrix_random(grid: &mut [[bool; 64]; 32]) {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if col % 3 == 0 || row.is_power_of_two() {
                grid[row][col] = true;
            }
        }
    }
}

pub(crate) fn copy_array(grid: &mut [[bool; 64]; 32], arr: &mut [bool; 64 * 32]) {
    let mut row = 0;
    let mut col = 0;
    for i in 0..arr.len() {
        if i % 64 == 0 && i > 0 {
            row += 1;
            col += 0;
        }
        grid[row][col] = arr[i];
        col += 1;
    }
}
pub(crate) fn clear_matrix(grid: &mut [[bool; 64]; 32]) {
    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|x| {
            if *x {
                *x = false;
            }
        })
    });
}

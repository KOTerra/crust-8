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
pub(crate) fn clear_matrix(grid: &mut [[bool; 64]; 32]) {
    grid.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|x| {
            if *x {
                *x = false;
            }
        })
    });
}

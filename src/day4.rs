pub fn a(input: &str) -> i32 {
    let mut columns = 0;
    let mut rows = 0;

    let mut grid = vec![];

    for line in input.trim().lines() {
        columns = columns.max(line.len());
        rows += 1;

        grid.extend(line.chars());
    }

    let idxtopos = |idx| (idx / columns, idx - (idx / columns) * columns);
    let postoidx = |(row, column)| row * rows + column;

    let adjustpos = |(row, column): (usize, usize), (row_offset, col_offset)| {
        Some((
            row.checked_add_signed(row_offset)
                .and_then(|r| (r < rows).then_some(r))?,
            column
                .checked_add_signed(col_offset)
                .and_then(|c| (c < columns).then_some(c))?,
        ))
    };

    let mut count = 0;

    for (idx, ch) in grid.iter().enumerate() {
        if *ch != '@' {
            continue;
        }
        let pos = idxtopos(idx);
        let neighbors = [
            adjustpos(pos, (-1, -1)),
            adjustpos(pos, (-1, 0)),
            adjustpos(pos, (-1, 1)),
            adjustpos(pos, (0, -1)),
            adjustpos(pos, (0, 1)),
            adjustpos(pos, (1, -1)),
            adjustpos(pos, (1, 0)),
            adjustpos(pos, (1, 1)),
        ];

        let num_neighbors: u32 = neighbors
            .into_iter()
            .map(|pos| pos.map_or(0, |pos| (grid[postoidx(pos)] == '@') as _))
            .sum();

        if num_neighbors < 4 {
            count += 1;
        }
    }
    count
}

pub fn b(input: &str) -> i32 {
    let mut columns = 0;
    let mut rows = 0;

    let mut grid = vec![];

    for line in input.trim().lines() {
        columns = columns.max(line.len());
        rows += 1;

        grid.extend(line.chars());
    }
    let mut grid_clone = grid.clone();

    let idxtopos = |idx| (idx / columns, idx - (idx / columns) * columns);
    let postoidx = |(row, column)| row * rows + column;

    let adjustpos = |(row, column): (usize, usize), (row_offset, col_offset)| {
        Some((
            row.checked_add_signed(row_offset)
                .and_then(|r| (r < rows).then_some(r))?,
            column
                .checked_add_signed(col_offset)
                .and_then(|c| (c < columns).then_some(c))?,
        ))
    };

    let mut removed = 0;

    let mut removed_in_iteration = 0;
    loop {
        for (idx, ch) in grid.iter().enumerate() {
            if *ch != '@' {
                continue;
            }
            let pos = idxtopos(idx);
            let neighbors = [
                adjustpos(pos, (-1, -1)),
                adjustpos(pos, (-1, 0)),
                adjustpos(pos, (-1, 1)),
                adjustpos(pos, (0, -1)),
                adjustpos(pos, (0, 1)),
                adjustpos(pos, (1, -1)),
                adjustpos(pos, (1, 0)),
                adjustpos(pos, (1, 1)),
            ];

            let num_neighbors: u32 = neighbors
                .into_iter()
                .map(|pos| pos.map_or(0, |pos| (grid[postoidx(pos)] == '@') as _))
                .sum();

            if num_neighbors < 4 {
                removed_in_iteration += 1;
                grid_clone[idx] = '.';
            }
        }
        removed += removed_in_iteration;
        if removed_in_iteration == 0 {
            break;
        }

        removed_in_iteration = 0;
        grid.clone_from(&grid_clone);
    }

    removed
}

const TEST: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

#[test]
fn test() {
    assert_eq!(a(TEST), 13);
    assert_eq!(b(TEST), 43);
}

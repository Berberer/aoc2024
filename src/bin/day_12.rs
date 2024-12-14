use std::collections::HashSet;

struct Farm {
    plots: Vec<Vec<char>>,
}

#[derive(Debug, PartialEq, Eq)]
struct FarmPlotInfo {
    plot_type: char,
    neighbor_north: Option<char>,
    neighbor_south: Option<char>,
    neighbor_west: Option<char>,
    neighbor_east: Option<char>,
}

impl Farm {
    fn get_plot_info(&self, x: usize, y: usize) -> FarmPlotInfo {
        let neighbor_north = if y > 0 {
            Some(self.plots[y - 1][x])
        } else {
            None
        };
        let neighbor_south = if y < (self.plots.len() - 1) {
            Some(self.plots[y + 1][x])
        } else {
            None
        };
        let neighbor_west = if x > 0 {
            Some(self.plots[y][x - 1])
        } else {
            None
        };
        let neighbor_east = if x < (self.plots[y].len() - 1) {
            Some(self.plots[y][x + 1])
        } else {
            None
        };

        FarmPlotInfo {
            plot_type: self.plots[y][x],
            neighbor_north,
            neighbor_south,
            neighbor_west,
            neighbor_east,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FarmPlotRegion {
    plot_type: char,
    perimeter: u32,
    area: u32,
    plot_coordinates: HashSet<(usize, usize)>,
    frontier_north: HashSet<(usize, usize)>,
    frontier_south: HashSet<(usize, usize)>,
    frontier_west: HashSet<(usize, usize)>,
    frontier_east: HashSet<(usize, usize)>,
}

fn parse_input(input: &str) -> Farm {
    Farm {
        plots: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn flood_to_region_border<'a>(
    plot_type: char,
    (x, y): (usize, usize),
    farm: &Farm,
    region_coordinates: &'a mut HashSet<(usize, usize)>,
    region_frontier_north: &'a mut HashSet<(usize, usize)>,
    region_frontier_south: &'a mut HashSet<(usize, usize)>,
    region_frontier_west: &'a mut HashSet<(usize, usize)>,
    region_frontier_east: &'a mut HashSet<(usize, usize)>,
) -> (
    &'a mut HashSet<(usize, usize)>,
    &'a mut HashSet<(usize, usize)>,
    &'a mut HashSet<(usize, usize)>,
    &'a mut HashSet<(usize, usize)>,
    &'a mut HashSet<(usize, usize)>,
) {
    let farm_plot_info = farm.get_plot_info(x, y);

    if farm_plot_info.plot_type == plot_type && !region_coordinates.contains(&(x, y)) {
        region_coordinates.insert((x, y));

        if farm_plot_info
            .neighbor_north
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(
                plot_type,
                (x, y - 1),
                farm,
                region_coordinates,
                region_frontier_north,
                region_frontier_south,
                region_frontier_west,
                region_frontier_east,
            );
        } else {
            region_frontier_north.insert((x, y));
        }
        if farm_plot_info
            .neighbor_south
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(
                plot_type,
                (x, y + 1),
                farm,
                region_coordinates,
                region_frontier_north,
                region_frontier_south,
                region_frontier_west,
                region_frontier_east,
            );
        } else {
            region_frontier_south.insert((x, y));
        }
        if farm_plot_info
            .neighbor_west
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(
                plot_type,
                (x - 1, y),
                farm,
                region_coordinates,
                region_frontier_north,
                region_frontier_south,
                region_frontier_west,
                region_frontier_east,
            );
        } else {
            region_frontier_west.insert((x, y));
        }
        if farm_plot_info
            .neighbor_east
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(
                plot_type,
                (x + 1, y),
                farm,
                region_coordinates,
                region_frontier_north,
                region_frontier_south,
                region_frontier_west,
                region_frontier_east,
            );
        } else {
            region_frontier_east.insert((x, y));
        }
    }
    (
        region_coordinates,
        region_frontier_north,
        region_frontier_south,
        region_frontier_west,
        region_frontier_east,
    )
}

fn calculate_fencing_perimeter(plot_info: &FarmPlotInfo) -> u32 {
    [
        plot_info.neighbor_north,
        plot_info.neighbor_south,
        plot_info.neighbor_west,
        plot_info.neighbor_east,
    ]
    .iter()
    .map(|p| {
        p.map(|c| if c != plot_info.plot_type { 1 } else { 0 })
            .unwrap_or(1)
    })
    .sum()
}

fn create_farm_plot_region_from_coordinates(
    plot_type: char,
    coordinates: HashSet<(usize, usize)>,
    frontier_north: HashSet<(usize, usize)>,
    frontier_south: HashSet<(usize, usize)>,
    frontier_west: HashSet<(usize, usize)>,
    frontier_east: HashSet<(usize, usize)>,
    farm: &Farm,
) -> FarmPlotRegion {
    let perimeter = coordinates
        .iter()
        .map(|(x, y)| calculate_fencing_perimeter(&farm.get_plot_info(*x, *y)))
        .sum();

    FarmPlotRegion {
        plot_type,
        perimeter,
        frontier_north,
        frontier_south,
        frontier_west,
        frontier_east,
        area: coordinates.len() as u32,
        plot_coordinates: coordinates,
    }
}

fn find_farm_plot_regions(farm: &Farm) -> Vec<FarmPlotRegion> {
    let mut regions = Vec::new();

    let mut covered_coordinates = HashSet::new();
    for y in 0..farm.plots.len() {
        for x in 0..farm.plots[y].len() {
            if !covered_coordinates.contains(&(x, y)) {
                let plot_type = farm.plots[y][x];
                let mut region_coordinates = HashSet::new();
                let mut region_frontier_north = HashSet::new();
                let mut region_frontier_south = HashSet::new();
                let mut region_frontier_west = HashSet::new();
                let mut region_frontier_east = HashSet::new();
                flood_to_region_border(
                    plot_type,
                    (x, y),
                    farm,
                    &mut region_coordinates,
                    &mut region_frontier_north,
                    &mut region_frontier_south,
                    &mut region_frontier_west,
                    &mut region_frontier_east,
                );

                covered_coordinates.extend(region_coordinates.clone());
                regions.push(create_farm_plot_region_from_coordinates(
                    plot_type,
                    region_coordinates,
                    region_frontier_north,
                    region_frontier_south,
                    region_frontier_west,
                    region_frontier_east,
                    farm,
                ));
            }
        }
    }

    regions
}

fn flood_unique_frontier_side<'a>(
    (x, y): (usize, usize),
    all_frontier_coordinates: &HashSet<(usize, usize)>,
    side_coordinates: &'a mut HashSet<(usize, usize)>,
) -> &'a mut HashSet<(usize, usize)> {
    if all_frontier_coordinates.contains(&(x, y)) && !side_coordinates.contains(&(x, y)) {
        side_coordinates.insert((x, y));

        if y > 0 {
            flood_unique_frontier_side((x, y - 1), all_frontier_coordinates, side_coordinates);
        }
        flood_unique_frontier_side((x, y + 1), all_frontier_coordinates, side_coordinates);
        if x > 0 {
            flood_unique_frontier_side((x - 1, y), all_frontier_coordinates, side_coordinates);
        }
        flood_unique_frontier_side((x + 1, y), all_frontier_coordinates, side_coordinates);
    }

    side_coordinates
}

fn count_unique_sides_of_frontier(all_frontier_coordinates: &HashSet<(usize, usize)>) -> u32 {
    let mut sides_count = 0;

    let mut covered_coordinates = HashSet::new();
    for coordinates in all_frontier_coordinates {
        if !covered_coordinates.contains(coordinates) {
            let mut side_coordinates = HashSet::new();
            flood_unique_frontier_side(
                *coordinates,
                all_frontier_coordinates,
                &mut side_coordinates,
            );
            covered_coordinates.extend(side_coordinates);
            sides_count += 1;
        }
    }

    sides_count
}

fn calculate_number_of_sides(farm_plot_region: &FarmPlotRegion) -> u32 {
    count_unique_sides_of_frontier(&farm_plot_region.frontier_north)
        + count_unique_sides_of_frontier(&farm_plot_region.frontier_south)
        + count_unique_sides_of_frontier(&farm_plot_region.frontier_west)
        + count_unique_sides_of_frontier(&farm_plot_region.frontier_east)
}

fn main() {
    let input = include_str!("../inputs/data_day_12.txt");
    let farm = parse_input(input);
    let farm_plot_regions = find_farm_plot_regions(&farm);

    // Solution for puzzle 1
    let fence_cost = farm_plot_regions
        .iter()
        .map(|r| r.area * r.perimeter)
        .sum::<u32>();
    println!("Fencing all regions of the farm will cost {fence_cost}");

    // Solution for puzzle 2
    let fence_cost_with_bulk_discount = farm_plot_regions
        .iter()
        .map(|r| r.area * calculate_number_of_sides(r))
        .sum::<u32>();
    println!("Fencing all regions of the farm with bulk discount will cost {fence_cost_with_bulk_discount}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighboring_plots() {
        let farm = Farm {
            plots: vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i'],
            ],
        };
        assert_eq!(
            farm.get_plot_info(0, 0),
            FarmPlotInfo {
                plot_type: 'a',
                neighbor_north: None,
                neighbor_south: Some('d'),
                neighbor_west: None,
                neighbor_east: Some('b'),
            }
        );
        assert_eq!(
            farm.get_plot_info(1, 1),
            FarmPlotInfo {
                plot_type: 'e',
                neighbor_north: Some('b'),
                neighbor_south: Some('h'),
                neighbor_west: Some('d'),
                neighbor_east: Some('f'),
            }
        );
        assert_eq!(
            farm.get_plot_info(2, 2),
            FarmPlotInfo {
                plot_type: 'i',
                neighbor_north: Some('f'),
                neighbor_south: None,
                neighbor_west: Some('h'),
                neighbor_east: None,
            }
        );
    }

    #[test]
    fn test_flood_to_region_border() {
        let farm = Farm {
            plots: vec![
                vec!['a', 'b', 'b'],
                vec!['b', 'b', 'b'],
                vec!['a', 'b', 'c'],
            ],
        };
        let mut coordinates = HashSet::new();
        let mut frontier_north = HashSet::new();
        let mut frontier_south = HashSet::new();
        let mut frontier_west = HashSet::new();
        let mut frontier_east = HashSet::new();
        flood_to_region_border(
            'b',
            (1, 0),
            &farm,
            &mut coordinates,
            &mut frontier_north,
            &mut frontier_south,
            &mut frontier_west,
            &mut frontier_east,
        );

        assert_eq!(
            coordinates,
            HashSet::from([(1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (1, 2),])
        );
        assert_eq!(frontier_north, HashSet::from([(0, 1), (1, 0), (2, 0)]));
        assert_eq!(frontier_south, HashSet::from([(0, 1), (1, 2), (2, 1)]));
        assert_eq!(frontier_west, HashSet::from([(0, 1), (1, 0), (1, 2)]));
        assert_eq!(frontier_east, HashSet::from([(2, 0), (2, 1), (1, 2)]));
    }

    #[test]
    fn test_calculate_fencing_perimeter() {
        let plot_info = FarmPlotInfo {
            plot_type: 'a',
            neighbor_north: Some('a'),
            neighbor_south: Some('a'),
            neighbor_west: Some('a'),
            neighbor_east: Some('a'),
        };
        assert_eq!(calculate_fencing_perimeter(&plot_info), 0);

        let plot_info = FarmPlotInfo {
            plot_type: 'a',
            neighbor_north: None,
            neighbor_south: Some('a'),
            neighbor_west: None,
            neighbor_east: Some('b'),
        };
        assert_eq!(calculate_fencing_perimeter(&plot_info), 3);
    }

    #[test]
    fn test_find_farm_plot_regions() {
        let farm = Farm {
            plots: vec![
                vec!['a', 'b', 'b'],
                vec!['b', 'b', 'b'],
                vec!['a', 'b', 'c'],
            ],
        };
        assert_eq!(
            find_farm_plot_regions(&farm),
            vec![
                FarmPlotRegion {
                    plot_type: 'a',
                    perimeter: 4,
                    area: 1,
                    plot_coordinates: HashSet::from([(0, 0)]),
                    frontier_north: HashSet::from([(0, 0)]),
                    frontier_south: HashSet::from([(0, 0)]),
                    frontier_west: HashSet::from([(0, 0)]),
                    frontier_east: HashSet::from([(0, 0)])
                },
                FarmPlotRegion {
                    plot_type: 'b',
                    perimeter: 12,
                    area: 6,
                    plot_coordinates: HashSet::from([
                        (1, 0),
                        (2, 0),
                        (0, 1),
                        (1, 1),
                        (2, 1),
                        (1, 2),
                    ]),
                    frontier_north: HashSet::from([(0, 1), (1, 0), (2, 0)]),
                    frontier_south: HashSet::from([(0, 1), (1, 2), (2, 1)]),
                    frontier_west: HashSet::from([(0, 1), (1, 0), (1, 2)]),
                    frontier_east: HashSet::from([(2, 0), (2, 1), (1, 2)])
                },
                FarmPlotRegion {
                    plot_type: 'a',
                    perimeter: 4,
                    area: 1,
                    plot_coordinates: HashSet::from([(0, 2)]),
                    frontier_north: HashSet::from([(0, 2)]),
                    frontier_south: HashSet::from([(0, 2)]),
                    frontier_west: HashSet::from([(0, 2)]),
                    frontier_east: HashSet::from([(0, 2)])
                },
                FarmPlotRegion {
                    plot_type: 'c',
                    perimeter: 4,
                    area: 1,
                    plot_coordinates: HashSet::from([(2, 2)]),
                    frontier_north: HashSet::from([(2, 2)]),
                    frontier_south: HashSet::from([(2, 2)]),
                    frontier_west: HashSet::from([(2, 2)]),
                    frontier_east: HashSet::from([(2, 2)])
                },
            ]
        );
    }

    #[test]
    fn find_count_unique_sides_of_frontier() {
        let frontier_coordinates = HashSet::from([
            (0, 1),
            (1, 1),
            (2, 0),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (6, 3),
        ]);
        assert_eq!(count_unique_sides_of_frontier(&frontier_coordinates), 4);
    }
}

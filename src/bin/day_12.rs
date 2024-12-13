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
}

fn parse_input(input: &str) -> Farm {
    Farm {
        plots: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

fn flood_to_region_border<'a>(
    plot_type: char,
    (x, y): (usize, usize),
    farm: &Farm,
    region_coordinates: &'a mut HashSet<(usize, usize)>,
) -> &'a mut HashSet<(usize, usize)> {
    let farm_plot_info = farm.get_plot_info(x, y);

    if farm_plot_info.plot_type == plot_type && !region_coordinates.contains(&(x, y)) {
        region_coordinates.insert((x, y));

        if farm_plot_info
            .neighbor_north
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(plot_type, (x, y - 1), farm, region_coordinates);
        }
        if farm_plot_info
            .neighbor_south
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(plot_type, (x, y + 1), farm, region_coordinates);
        }
        if farm_plot_info
            .neighbor_west
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(plot_type, (x - 1, y), farm, region_coordinates);
        }
        if farm_plot_info
            .neighbor_east
            .map(|c| c == plot_type)
            .unwrap_or(false)
        {
            flood_to_region_border(plot_type, (x + 1, y), farm, region_coordinates);
        }

        region_coordinates
    } else {
        region_coordinates
    }
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
    farm: &Farm,
) -> FarmPlotRegion {
    let perimeter = coordinates
        .iter()
        .map(|(x, y)| calculate_fencing_perimeter(&farm.get_plot_info(*x, *y)))
        .sum();

    FarmPlotRegion {
        plot_type,
        perimeter,
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
                flood_to_region_border(plot_type, (x, y), farm, &mut region_coordinates);

                covered_coordinates.extend(region_coordinates.clone());
                regions.push(create_farm_plot_region_from_coordinates(
                    plot_type,
                    region_coordinates,
                    farm,
                ));
            }
        }
    }

    regions
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
        flood_to_region_border('b', (1, 0), &farm, &mut coordinates);
        assert_eq!(
            coordinates,
            HashSet::from([(1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (1, 2),])
        );
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
                    plot_coordinates: HashSet::from([(0, 0)])
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
                    ])
                },
                FarmPlotRegion {
                    plot_type: 'a',
                    perimeter: 4,
                    area: 1,
                    plot_coordinates: HashSet::from([(0, 2)])
                },
                FarmPlotRegion {
                    plot_type: 'c',
                    perimeter: 4,
                    area: 1,
                    plot_coordinates: HashSet::from([(2, 2)])
                },
            ]
        );
    }
}

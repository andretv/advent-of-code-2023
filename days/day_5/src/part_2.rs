use rayon::prelude::*;

pub fn solution(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = get_seeds(lines.next().expect("Seeds line should always exist"));
    // Consumes empty line
    lines.next();

    let seeds_to_soil = get_map(&mut lines, "seed-to-soil");
    let soil_to_fertilizer = get_map(&mut lines, "soil-to-fertilizer");
    let fertilizer_to_water = get_map(&mut lines, "fertilizer-to-water");
    let water_to_light = get_map(&mut lines, "water-to-light");
    let light_to_temperature = get_map(&mut lines, "light-to-temperature");
    let temperature_to_humidity = get_map(&mut lines, "temperature-to-humidity");
    let humidity_to_location = get_map(&mut lines, "humidity-to-location");

    seeds
        .par_iter()
        .map(|&seed| {
            let soil_code = get_code(&seed, &seeds_to_soil).unwrap_or(seed);
            let fertilizer_code = get_code(&soil_code, &soil_to_fertilizer).unwrap_or(soil_code);
            let water_code =
                get_code(&fertilizer_code, &fertilizer_to_water).unwrap_or(fertilizer_code);
            let light_code = get_code(&water_code, &water_to_light).unwrap_or(water_code);
            let temperature_code =
                get_code(&light_code, &light_to_temperature).unwrap_or(light_code);
            let humidity_code =
                get_code(&temperature_code, &temperature_to_humidity).unwrap_or(temperature_code);
            let location_code =
                get_code(&humidity_code, &humidity_to_location).unwrap_or(humidity_code);

            location_code
        })
        .min()
        .expect("Locations should always have at least one value")
}

///
/// Given a source code it tries to find the corresponding destination code.
///
fn get_code(source: &u64, destination: &Vec<[u64; 3]>) -> Option<u64> {
    for seed_to_soil in destination {
        let [destination_range_start, source_range_start, range_length] = &seed_to_soil;
        let source_range_end = source_range_start + range_length;

        if (*source_range_start..source_range_end).contains(source) {
            let diff = source - source_range_start;
            let destination_code = diff + destination_range_start;
            return Some(destination_code);
        }
    }

    None
}

///
/// Generates a vector with all maps.
///
fn get_map<'a>(input: &mut impl Iterator<Item = &'a str>, key: &str) -> Vec<[u64; 3]> {
    let mut maps: Vec<_> = vec![];

    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        if line.starts_with(key) {
            continue;
        }

        let mut split = line.split_whitespace().enumerate();
        let mut map: [u64; 3] = [0; 3];

        while let Some((index, number)) = split.next() {
            map[index] = number.parse().expect("Number should always be parsable");
        }

        maps.push(map);
    }

    maps
}

///
/// Generates the seeds vector.
///
fn get_seeds(input: &str) -> Vec<u64> {
    let mut seeds: Vec<u64> = vec![];

    let temp_seeds: Vec<u64> = input
        .split_once(":")
        .expect("Should always split on `:`")
        .1
        .split_whitespace()
        .map(|number| number.parse().expect("Number should always be parsable"))
        .collect();

    for pair in temp_seeds.chunks(2) {
        let start = pair[0];
        let length = pair[1];

        for seed in start..(start + length) {
            seeds.push(seed);
        }
    }

    seeds
}

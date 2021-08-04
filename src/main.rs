use crate::day01::{cumulate_frequency_adjustments, find_first_repeated_frequency};
use crate::day02::{differing_letters_of_correct_boxes, product_of_2_and_3_counts};
use crate::day03::{id_of_non_overlapping_claim, overlapping_claim_count};
use crate::day04::{strategy_one, strategy_two};
use crate::day05::{length_of_shortest_possible_polymer, remaining_units_after_reaction};
use crate::day06::{
    size_of_area_with_max_total_distance_to_all_coords, size_of_largest_finite_area,
};
use crate::day07::{count_seconds, order_of_steps};
use crate::day08::{input_metadata_sum, input_value};
use crate::day09::high_score;
use crate::day10::message;
use crate::day11::{largest_total_power_3x3_square, largest_total_power_variable_size_square};
use crate::day12::{number_of_plants_after_20_gens, number_of_plants_after_generations};
use crate::day13::{location_of_first_crash, location_of_last_cart};
use crate::day14::{recipe_count_until_this_score_appears, score_of_10_recipes_after};
use crate::day15::Grid;

use line_reader::read_file_to_lines;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    day01();
    day02();
    day03();
    day04();
    day05();
    day06();
    day07();
    day08();
    day09();
    day10();
    day11();
    day12();
    day13();
    day14();
    day15();
}

fn day01() {
    assert_eq!(
        cumulate_frequency_adjustments(&read_file_to_lines("input/day01.txt")),
        454
    );
    assert_eq!(
        find_first_repeated_frequency(&read_file_to_lines("input/day01.txt")),
        566
    );
}

fn day02() {
    assert_eq!(
        product_of_2_and_3_counts(&read_file_to_lines("input/day02.txt")),
        7936
    );
    assert_eq!(
        differing_letters_of_correct_boxes(&read_file_to_lines("input/day02.txt")),
        "lnfqdscwjyteorambzuchrgpx"
    );
}

fn day03() {
    assert_eq!(
        overlapping_claim_count(&read_file_to_lines("input/day03.txt")),
        113576
    );
    assert_eq!(
        id_of_non_overlapping_claim(&read_file_to_lines("input/day03.txt")),
        825
    );
}

fn day04() {
    assert_eq!(65489, strategy_one(&read_file_to_lines("input/day04.txt")));
    assert_eq!(3852, strategy_two(&read_file_to_lines("input/day04.txt")));
}

fn day05() {
    assert_eq!(
        9462,
        remaining_units_after_reaction(&read_file_to_lines("input/day05.txt")[0])
    );
    assert_eq!(
        4952,
        length_of_shortest_possible_polymer(&read_file_to_lines("input/day05.txt")[0])
    );
}

fn day06() {
    assert_eq!(
        4589,
        size_of_largest_finite_area(read_file_to_lines("input/day06.txt"))
    );
    assert_eq!(
        40252,
        size_of_area_with_max_total_distance_to_all_coords(
            read_file_to_lines("input/day06.txt"),
            10_000
        )
    );
}

fn day07() {
    assert_eq!(
        "JNOIKSYABEQRUVWXGTZFDMHLPC",
        order_of_steps(&read_file_to_lines("input/day07.txt"))
    );
    assert_eq!(
        1099,
        count_seconds(&read_file_to_lines("input/day07.txt"), 5, 60)
    );
}

fn day08() {
    assert_eq!(
        42146,
        input_metadata_sum(&read_file_to_lines("input/day08.txt")[0])
    );
    assert_eq!(
        26753,
        input_value(&read_file_to_lines("input/day08.txt")[0])
    );
}

fn day09() {
    assert_eq!(374690, high_score(477, 70851));
    assert_eq!(3_009_951_158, high_score(477, 7_085_100));
}

fn day10() {
    let input = &read_file_to_lines("input/day10.txt");
    assert_eq!(10511, message(input).1);
}

fn day11() {
    assert_eq!((28, 235, 87), largest_total_power_3x3_square(8199));
    assert_eq!(
        (119, 234, 272, 18),
        largest_total_power_variable_size_square(8199)
    );
}

fn day12() {
    assert_eq!(
        2063,
        number_of_plants_after_20_gens(&read_file_to_lines("input/day12.txt"))
    );
    assert_eq!(
        1_600_000_000_328,
        number_of_plants_after_generations(&read_file_to_lines("input/day12.txt"), 50_000_000_000)
    );
}

fn day13() {
    assert_eq!(
        (102, 114),
        location_of_first_crash(&read_file_to_lines("input/day13.txt"))
    );
    assert_eq!(
        (146, 87),
        location_of_last_cart(&read_file_to_lines("input/day13.txt"))
    );
}

fn day14() {
    assert_eq!("1411383621", score_of_10_recipes_after(760_221));
    assert_eq!(20177474, recipe_count_until_this_score_appears("760_221"));
}

fn day15() {
    assert_eq!(
        207_059,
        Grid::from(&read_file_to_lines("input/day15.txt")).play_until_no_enemies_remain()
    );
    assert_eq!(
        49_120,
        Grid::from(&read_file_to_lines("input/day15.txt"))
            .play_with_increasing_elf_attack_power_until_elves_win_without_a_single_loss()
    );
}

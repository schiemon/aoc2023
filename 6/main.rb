#!/usr/bin/env ruby

def parse_numbers(line)
  unless line.is_a?(String)
    raise ArgumentError, "line must be a String"
  end
  line
    .strip
    .split(" ")
    .filter { |number_str| number_str != "" }
    .map { |number_str| number_str.to_i }
end

def get_num_winning_options(race_time, record_distance)
  race_time = race_time.to_f
  record_distance = record_distance.to_f

  holding_time_to_distance = -> (holding_time) { holding_time * (race_time - holding_time) }
  optimal_holding_time = race_time / 2

  if record_distance >= holding_time_to_distance.(optimal_holding_time)
    return 0
  end

  d = ((race_time / 2) ** 2 - (record_distance + 1)) ** 0.5

  winning_holding_time_left_bound = (race_time / 2 - d).ceil
  winning_holding_time_right_bound = (race_time / 2 + d).floor

  winning_holding_time_right_bound - winning_holding_time_left_bound + 1
end

def part1(f)
  unless f.is_a?(File)
    raise ArgumentError, "file must be a File"
  end

  record_times = parse_numbers(f.readline.delete_prefix("Time:"))
  distances = parse_numbers(f.readline.delete_prefix("Distance:"))

  if record_times.length != distances.length
    raise ArgumentError, "given record times and distances must be the same length"
  end

  record_times
    .zip(distances)
    .map { |(record_time, distance)| get_num_winning_options(record_time, distance) }
    .reduce(:*)
end

def part2(f)
  unless f.is_a?(File)
    raise ArgumentError, "file must be a File"
  end

  record_time = f.readline.delete_prefix("Time:").delete(" ").to_i
  distance = f.readline.delete_prefix("Distance:").delete(" ").to_i

  get_num_winning_options(record_time, distance)
end

def main
  if ARGV.length < 1
    puts "Usage: main.rb <input file>"
    exit
  end

  File.open(ARGV[0], "r") do |f|
    puts "Part 1: " + part1(f).to_s
  end

  File.open(ARGV[0], "r") do |f|
    puts "Part 2: " + part2(f).to_s
  end
end

main
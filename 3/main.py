#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
from typing import Optional, Tuple, IO, AnyStr


def is_symbol(ch: str) -> bool:
    return not ch.isdigit() and ch != "."


def get_line_triples(file: IO[AnyStr]) -> Tuple[str, str, str]:
    current_line = file.readline().strip()

    if not current_line:
        return

    begin_and_end_line = "." * len(current_line)
    prev_line = begin_and_end_line

    last_line = False
    while not last_line:
        next_line = file.readline().strip()

        if not next_line:
            next_line = begin_and_end_line
            last_line = True

        yield prev_line, current_line, next_line

        prev_line, current_line = current_line, next_line


# Part 1 ----------------------------------------------------------------------

def get_sum(prev_line: str, current_line: str, next_line: str) -> int:
    prev_line = "." + prev_line + "."
    current_line = "." + current_line + "."
    next_line = "." + next_line + "."

    current_digit = ""
    should_count = False
    sum_of_numbers_in_current_line = 0

    for (i, ch) in enumerate((current_line + ".")):
        if ch.isdigit():
            current_digit += ch
            for neighbor_line in [prev_line, current_line, next_line]:
                should_count = (should_count or any(map(lambda j: is_symbol(neighbor_line[j]), [i - 1, i, i + 1])))
        else:
            if should_count:
                sum_of_numbers_in_current_line += int(current_digit)
                should_count = False
            current_digit = ""

    return sum_of_numbers_in_current_line


def part1(file: IO[AnyStr]) -> int:
    answer = 0
    for prev_line, current_line, next_line in get_line_triples(file):
        answer += get_sum(prev_line, current_line, next_line)

    return answer


# Part 2 ----------------------------------------------------------------------

def get_num(line: str, i: int) -> Optional[Tuple[int, int, int]]:
    if not line[i].isdigit():
        return None

    l = i

    while (l - 1) >= 0 and line[l - 1].isdigit():
        l -= 1

    r = i

    while (r + 1) < len(line) and line[r + 1].isdigit():
        r += 1

    return l, r, int(line[l:r + 1])


def part2(file: IO[AnyStr]) -> int:
    answer = 0
    for prev_line, current_line, next_line in get_line_triples(file):
        for i, ch in enumerate(current_line):
            if ch != "*":
                continue

            neighbor_lines = [prev_line, current_line, next_line]
            nums = set()
            for di, dl in [
                (-1, -1), (0, -1), (1, -1),
                (-1, 0), (1, 0),
                (-1, 1), (0, 1), (1, 1)
            ]:

                num = get_num(neighbor_lines[dl + 1], i + di)

                if num is not None:
                    nums.add(num)

            if len(nums) != 2:
                continue

            (_, _, first_num), (_, _, second_num) = nums

            answer += first_num * second_num

    return answer


def main():
    input_file_name = sys.argv[1]

    with open(input_file_name, 'r') as file:
        # print(part1(file))
        print(part2(file))


if __name__ == "__main__":
    main()

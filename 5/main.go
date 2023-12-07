package main

import (
	"bufio"
	"errors"
	"sort"
	"strconv"
	"strings"
)

import (
	"./utils"
	"fmt"
	"log"
	"os"
)

type CategoryToCategoryAssignment struct {
	fromRange utils.SeedRange
	toRange   utils.SeedRange
}

type CategoryToCategoryMap struct {
	fromCategory string
	toCategory   string

	// Their `fromRange`s should be pairwise disjoint.
	// They should be sorted by `fromRange.start`.
	assignments []CategoryToCategoryAssignment
}

func parseInitialSeeds(inputFileScanner *bufio.Scanner) ([]int, error) {
	if !(inputFileScanner.Scan()) {
		return []int{}, errors.New("cannot read first line (initial seeds)")
	}

	seedLine := inputFileScanner.Text()

	initialSeedsStr := strings.Split(
		strings.TrimSpace(
			strings.Split(seedLine, ":")[1],
		),
		" ",
	)

	initialSeeds := make([]int, len(initialSeedsStr))

	for i, seedStr := range initialSeedsStr {
		initialSeeds[i], _ = strconv.Atoi(seedStr)
	}

	return initialSeeds, nil
}

// Expects the scanner to already have scanned the header line.
func parseCategoryToCategoryMap(inputFileScanner *bufio.Scanner) (*CategoryToCategoryMap, error) {
	categoryNames := strings.Split(strings.Split(inputFileScanner.Text(), " ")[0], "-to-")

	if len(categoryNames) != 2 {
		return nil, errors.New("invalid category to category map header")
	}

	categoryToCategoryMap := CategoryToCategoryMap{}

	categoryToCategoryMap.fromCategory = categoryNames[0]
	categoryToCategoryMap.toCategory = categoryNames[1]

	for inputFileScanner.Scan() {
		assignmentString := inputFileScanner.Text()

		if (assignmentString) == "" {
			break
		}

		assignmentRange := strings.Split(assignmentString, " ")

		if len(assignmentRange) != 3 {
			return nil, errors.New("invalid assignment range")
		}

		destinationRangeStart, errorDestinationRange := strconv.Atoi(assignmentRange[0])
		sourceRangeStart, errorSourceRange := strconv.Atoi(assignmentRange[1])
		rangeLength, errorRangeLength := strconv.Atoi(assignmentRange[2])

		if errorSourceRange != nil || errorDestinationRange != nil || errorRangeLength != nil {
			return nil, errors.New("cannot parse assignment range")
		}

		categoryToCategoryMap.assignments = append(
			categoryToCategoryMap.assignments,
			CategoryToCategoryAssignment{
				fromRange: utils.NewRange(sourceRangeStart, sourceRangeStart+rangeLength-1),
				toRange:   utils.NewRange(destinationRangeStart, destinationRangeStart+rangeLength-1),
			})
	}

	sort.Slice(categoryToCategoryMap.assignments, func(i, j int) bool {
		return categoryToCategoryMap.assignments[i].fromRange.Start() < categoryToCategoryMap.assignments[j].fromRange.Start()
	})

	return &categoryToCategoryMap, nil
}

func parseCategoryToCategoryMaps(inputFileScanner *bufio.Scanner) ([]CategoryToCategoryMap, error) {
	var categoryMaps []CategoryToCategoryMap

	inputFileScanner.Scan()

	for inputFileScanner.Scan() {
		if (inputFileScanner.Text()) == "" {
			break
		}

		categoryToCategoryMap, err := parseCategoryToCategoryMap(inputFileScanner)

		if err != nil {
			return nil, err
		}

		categoryMaps = append(categoryMaps, *categoryToCategoryMap)
	}

	return categoryMaps, nil
}

func mapRanges(seedRanges []utils.SeedRange, categoryMaps []CategoryToCategoryMap) []utils.SeedRange {
	if len(categoryMaps) == 0 {
		return seedRanges
	}

	var mappedRanges []utils.SeedRange
	firstCategoryMap := categoryMaps[0]

	for _, seedRange := range seedRanges {
		seedRangesLeftToMap := []utils.SeedRange{seedRange}

		for len(seedRangesLeftToMap) > 0 {
			seedRangeToMap := seedRangesLeftToMap[0]
			seedRangesLeftToMap = seedRangesLeftToMap[1:]

			mapped := false

			lowerBoundAssignmentIndex := 0
			upperBoundAssignmentIndex := len(firstCategoryMap.assignments) - 1
			assignmentIndex := -1

			for lowerBoundAssignmentIndex <= upperBoundAssignmentIndex {
				m := (lowerBoundAssignmentIndex + upperBoundAssignmentIndex) / 2

				if firstCategoryMap.assignments[m].fromRange.Start() <= seedRangeToMap.Start() {
					lowerBoundAssignmentIndex = m + 1
					assignmentIndex = m
				} else if firstCategoryMap.assignments[m].fromRange.Start() > seedRangeToMap.Start() {
					upperBoundAssignmentIndex = m - 1
				}
			}

			if assignmentIndex == -1 {
				assignmentIndex = 0
			}

			for _, assignment := range firstCategoryMap.assignments[assignmentIndex:] {
				intersection := assignment.fromRange.Intersection(seedRangeToMap)

				if intersection.Empty() {
					break
				}

				beforeIntersection, afterIntersection := seedRangeToMap.Difference(intersection)

				mappedRanges = append(mappedRanges, utils.NewRange(
					assignment.toRange.Start()+(intersection.Start()-assignment.fromRange.Start()),
					assignment.toRange.Start()+(intersection.End()-assignment.fromRange.Start()),
				))

				if !beforeIntersection.Empty() {
					seedRangesLeftToMap = append(seedRangesLeftToMap, beforeIntersection)
				}

				if !afterIntersection.Empty() {
					seedRangesLeftToMap = append(seedRangesLeftToMap, afterIntersection)
				}

				mapped = true
				break
			}

			if !mapped {
				mappedRanges = append(mappedRanges, seedRangeToMap)
			}
		}
	}

	return mapRanges(mappedRanges, categoryMaps[1:])
}

func part1(inputFileScanner *bufio.Scanner) (int, error) {
	seeds, _ := parseInitialSeeds(inputFileScanner)
	categoryMaps, err := parseCategoryToCategoryMaps(inputFileScanner)

	if err != nil {
		return 0, err
	}

	var seedRanges []utils.SeedRange

	for _, seed := range seeds {
		seedRanges = append(seedRanges, utils.NewPointRange(seed))
	}

	return getSmallestLocation(seedRanges, categoryMaps), nil
}

func part2(inputFileScanner *bufio.Scanner) (int, error) {
	seeds, _ := parseInitialSeeds(inputFileScanner)
	categoryMaps, err := parseCategoryToCategoryMaps(inputFileScanner)

	if err != nil {
		return 0, err
	}

	var seedRanges []utils.SeedRange

	for i := 0; i < len(seeds)-1; i += 2 {
		seedRanges = append(seedRanges, utils.NewRange(seeds[i], seeds[i]+seeds[i+1]-1))
	}

	return getSmallestLocation(seedRanges, categoryMaps), nil
}

func getSmallestLocation(seedRanges []utils.SeedRange, categoryMaps []CategoryToCategoryMap) int {
	locations := mapRanges(seedRanges, categoryMaps)

	minimumLocation := locations[0].Start()

	for _, location := range locations[1:] {
		if location.Start() < minimumLocation {
			minimumLocation = location.Start()
		}
	}

	return minimumLocation
}

func main() {
	if len(os.Args) < 2 {
		log.Fatal("No input file specified.")
	}

	inputFileName := os.Args[1]

	file, err := os.Open(inputFileName)

	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)

	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(file)

	answer, err := part2(scanner)

	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(answer)
}

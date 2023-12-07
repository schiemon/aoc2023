package utils

type SeedRange struct {
	// Both inclusive
	start int
	end   int
}

func EmptyRange() SeedRange {
	return SeedRange{
		start: 0,
		end:   -1,
	}
}

func NewRange(start int, end int) SeedRange {
	return SeedRange{
		start: start,
		end:   end,
	}
}

func NewPointRange(start int) SeedRange {
	return NewRange(start, start)
}

func (r SeedRange) Start() int {
	return r.start
}

func (r SeedRange) End() int {
	return r.end
}

func (r SeedRange) Empty() bool {
	return r.start > r.end
}

func (r SeedRange) Intersection(other SeedRange) SeedRange {
	if r.start > other.end || other.start > r.end {
		return EmptyRange()
	}

	return SeedRange{
		start: max(r.start, other.start),
		end:   min(r.end, other.end),
	}
}

func (r SeedRange) Difference(other SeedRange) (SeedRange, SeedRange) {
	intersection := r.Intersection(other)

	if intersection.Empty() {
		return EmptyRange(), EmptyRange()
	}

	if intersection.start == r.start && intersection.end == r.end {
		return EmptyRange(), EmptyRange()
	}

	if intersection.start == r.start {
		return EmptyRange(), NewRange(intersection.end+1, r.end)
	}

	if intersection.end == r.end {
		return NewRange(r.start, intersection.start-1), EmptyRange()
	}

	return NewRange(r.start, intersection.start-1), NewRange(intersection.end+1, r.end)
}

package day02

import "testing"

func TestIsIncreasing(t *testing.T) {
	levels := []int{1, 2, 3, 4, 5}
	if isIncreasing(levels) == false {
		t.Errorf("%v should be increasing", levels)
	}

	levels = []int{1, 2, 2, 3, 4}
	if isIncreasing(levels) == false {
		t.Errorf("%v should be increasing", levels)
	}

	levels = []int{1, 1, 1, 1, 1}
	if isIncreasing(levels) == false {
		t.Errorf("%v should be increasing", levels)
	}

	levels = []int{5, 4, 3, 2, 1}
	if isIncreasing(levels) == true {
		t.Errorf("%v should not be increasing", levels)
	}

	levels = []int{1, 2, 5, 3, 4}
	if isIncreasing(levels) == true {
		t.Errorf("%v should not be increasing", levels)
	}
}

func TestIsDecreasing(t *testing.T) {
	levels := []int{5, 4, 3, 2, 1}
	if isDecreasing(levels) == false {
		t.Errorf("%v should be decreasing", levels)
	}

	levels = []int{5, 4, 4, 3, 2}
	if isDecreasing(levels) == false {
		t.Errorf("%v should be decreasing", levels)
	}

	levels = []int{1, 1, 1, 1, 1}
	if isDecreasing(levels) == false {
		t.Errorf("%v should be decreasing", levels)
	}

	levels = []int{1, 2, 3, 4, 5}
	if isDecreasing(levels) == true {
		t.Errorf("%v should not be decreasing", levels)
	}

	levels = []int{5, 4, 1, 3, 2}
	if isDecreasing(levels) == true {
		t.Errorf("%v should not be decreasing", levels)
	}
}

func TestHasGradualDifference(t *testing.T) {
	levels := []int{1, 2, 3, 4, 5}
	if hasGradualDifference(levels) == false {
		t.Errorf("%v has gradual difference", levels)
	}

	levels = []int{5, 4, 3, 2, 1}
	if hasGradualDifference(levels) == false {
		t.Errorf("%v has gradual difference", levels)
	}

	levels = []int{1, 4, 7, 10, 13}
	if hasGradualDifference(levels) == false {
		t.Errorf("%v has gradual difference", levels)
	}

	levels = []int{1, 2, 7, 8, 9}
	if hasGradualDifference(levels) == true {
		t.Errorf("%v has difference of 5 > 3", levels)
	}

	levels = []int{1, 1, 1, 1, 1}
	if hasGradualDifference(levels) == true {
		t.Errorf("%v has difference of 0 < 1", levels)
	}
}

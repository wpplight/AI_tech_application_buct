package algorithms

import (
	mappkg "wayfind/backend/map"
)

type SearchResult struct {
	Path      []mappkg.Point
	Expanded  int
	Time      int64
	Distance  int
	Algorithm string
	Found     bool
}

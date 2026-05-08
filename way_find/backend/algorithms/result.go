package algorithms

import (
	mappkg "wayfind/backend/map"
)

type SearchResult struct {
	Path      []mappkg.Point `json:"path"`
	Expanded  int           `json:"expanded"`
	Time      int64         `json:"time"`
	Distance  int           `json:"distance"`
	Algorithm string        `json:"algorithm"`
	Found     bool          `json:"found"`
}

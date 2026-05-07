package algorithms

import (
	mappkg "wayfind/backend/map"
)

type SearchState int

const (
	StateReady SearchState = iota
	StateRunning
	StateFound
	StateNotFound
)

type StepResult struct {
	State      SearchState
	Current    mappkg.Point
	Neighbors  []mappkg.Point
	Added      []mappkg.Point
	Pruned     []mappkg.Point
	Path       []mappkg.Point
	Visited    []mappkg.Point
	Distance   int
	Expanded   int
	StepsTaken int
}

type Searcher interface {
	Initialize(m *mappkg.Map)
	Step() *StepResult
	IsDone() bool
	GetResult() *SearchResult
	GetCurrentPath() []mappkg.Point
	GetVisited() []mappkg.Point
	GetShortestPath() []mappkg.Point
}

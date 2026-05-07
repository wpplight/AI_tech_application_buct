package queue

import (
	mappkg "wayfind/backend/map"
)

type BFSItem struct {
	Point mappkg.Point
	Step  int
}

type BFSQueue struct {
	items []BFSItem
}

func NewBFSQueue() *BFSQueue {
	return &BFSQueue{
		items: make([]BFSItem, 0),
	}
}

func (q *BFSQueue) Push(item BFSItem) {
	q.items = append(q.items, item)
}

func (q *BFSQueue) Pop() BFSItem {
	if len(q.items) == 0 {
		return BFSItem{}
	}
	item := q.items[0]
	q.items = q.items[1:]
	return item
}

func (q *BFSQueue) IsEmpty() bool {
	return len(q.items) == 0
}

func (q *BFSQueue) Size() int {
	return len(q.items)
}

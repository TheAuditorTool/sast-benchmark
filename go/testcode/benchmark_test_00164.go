package testcode

import (
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00164(w http.ResponseWriter, r *http.Request) {
	items := []map[string]interface{}{
		{"id": 1, "name": "Widget A", "price": 19.99},
		{"id": 2, "name": "Widget B", "price": 29.99},
		{"id": 3, "name": "Widget C", "price": 9.99},
		{"id": 4, "name": "Widget D", "price": 49.99},
		{"id": 5, "name": "Widget E", "price": 14.99},
	}

	rand.Seed(time.Now().UnixNano())
	rand.Shuffle(len(items), func(i, j int) {
		items[i], items[j] = items[j], items[i]
	})

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"items":   items,
		"display": "randomized",
	})
}

package testcode

import (
	"net/http"
)

type benchmarkTest00648Item struct {
	ID    string
	Name  string
	Price float64
}

func (it *benchmarkTest00648Item) Process() map[string]interface{} {
	return map[string]interface{}{
		"id":    it.ID,
		"name":  it.Name,
		"price": it.Price,
	}
}

var benchmarkTest00648Items = map[string]*benchmarkTest00648Item{
	"prod-001": {ID: "prod-001", Name: "Widget A", Price: 9.99},
	"prod-002": {ID: "prod-002", Name: "Widget B", Price: 14.99},
	"prod-003": {ID: "prod-003", Name: "Widget C", Price: 4.99},
}

func BenchmarkTest00648(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	if id == "" {
		http.Error(w, "id parameter required", http.StatusBadRequest)
		return
	}

	item, ok := benchmarkTest00648Items[id]
	if !ok {
		http.Error(w, "item not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, item.Process())
}

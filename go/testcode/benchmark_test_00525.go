package testcode

import (
	"net/http"
	"strconv"
)

var benchmarkTest00525DataArray = []string{
	"alpha", "bravo", "charlie", "delta", "echo",
	"foxtrot", "golf", "hotel", "india", "juliet",
}

func BenchmarkTest00525(w http.ResponseWriter, r *http.Request) {
	indexStr := r.URL.Query().Get("index")
	i, _ := strconv.Atoi(indexStr)

	result := benchmarkTest00525DataArray[i]

	RespondJSON(w, http.StatusOK, map[string]string{"value": result})
}

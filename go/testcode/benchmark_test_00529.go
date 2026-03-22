package testcode

import (
	"net/http"
	"strconv"
)

var benchmarkTest00529DataArray = []string{
	"alpha", "bravo", "charlie", "delta", "echo",
	"foxtrot", "golf", "hotel", "india", "juliet",
}

func BenchmarkTest00529(w http.ResponseWriter, r *http.Request) {
	indexStr := r.URL.Query().Get("index")
	i, err := strconv.Atoi(indexStr)
	if err != nil {
		http.Error(w, "invalid index parameter", http.StatusBadRequest)
		return
	}

	if i < 0 || i >= len(benchmarkTest00529DataArray) {
		http.Error(w, "index out of range", http.StatusBadRequest)
		return
	}

	result := benchmarkTest00529DataArray[i]

	RespondJSON(w, http.StatusOK, map[string]string{"value": result})
}

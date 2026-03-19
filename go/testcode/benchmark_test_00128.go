package testcode

import (
	"fmt"
	"net/http"
	"strconv"
)

func BenchmarkTest00128(w http.ResponseWriter, r *http.Request) {
	userInput := r.URL.Query().Get("count")
	intVal, err := strconv.Atoi(userInput)
	if err != nil {
		http.Error(w, "invalid number", http.StatusBadRequest)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>Count: %d</p>", intVal)
}

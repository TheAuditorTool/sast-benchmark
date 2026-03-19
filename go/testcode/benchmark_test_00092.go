package testcode

import (
	"fmt"
	"net/http"
	"os"
	"strconv"
)

func BenchmarkTest00092(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	id, err := strconv.Atoi(idStr)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	path := fmt.Sprintf("/var/data/reports/%d.json", id)
	content, err := os.ReadFile(path)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	w.Write(content)
}

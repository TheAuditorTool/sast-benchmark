package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest01335(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var status string
	err := DB.QueryRow("SELECT status FROM jobs WHERE id = ?", id).Scan(&status)
	if err != nil {
		wd, _ := os.Getwd()
		fmt.Fprintf(w, "working dir: %s, error: %v", wd, err)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": status})
}

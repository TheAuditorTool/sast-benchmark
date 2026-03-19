package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00129(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("id")
	var displayName string
	err := DB.QueryRow("SELECT display_name FROM users WHERE id = ?", userID).Scan(&displayName)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}
	w.Header().Set("Content-Type", "text/html")
	fmt.Fprintf(w, "<p>%s</p>", displayName)
}

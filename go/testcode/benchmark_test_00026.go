package testcode

import (
	"net/http"
	"regexp"
)

func BenchmarkTest00026(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	validID := regexp.MustCompile("^[a-zA-Z0-9]+$")
	if !validID.MatchString(id) {
		http.Error(w, "invalid id format", http.StatusBadRequest)
		return
	}
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

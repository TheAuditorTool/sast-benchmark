package testcode

import (
	"net/http"
	"regexp"
)

var benchmarkTest00693SafePattern = regexp.MustCompile(`^[a-zA-Z0-9_]{1,64}$`)

func BenchmarkTest00693(w http.ResponseWriter, r *http.Request) {
	userID := r.URL.Query().Get("user_id")
	var storedCategory string
	err := DB.QueryRow("SELECT category FROM user_prefs WHERE user_id = ?", userID).Scan(&storedCategory)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	if !benchmarkTest00693SafePattern.MatchString(storedCategory) {
		http.Error(w, "invalid category", http.StatusBadRequest)
		return
	}
	rows, err := DB.Query("SELECT id, name FROM items WHERE category = ?", storedCategory)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

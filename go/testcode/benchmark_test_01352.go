package testcode

import (
	"net/http"
)

func BenchmarkTest01352(w http.ResponseWriter, r *http.Request) {
	userID := r.Header.Get("X-User-ID")
	rows, err := DB.Query("SELECT name, email FROM users WHERE id = ?", userID)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	var results []map[string]string
	for rows.Next() {
		var name, email string
		if err := rows.Scan(&name, &email); err != nil {
			continue
		}
		results = append(results, map[string]string{"name": name, "email": email})
	}
	RespondJSON(w, http.StatusOK, results)
}

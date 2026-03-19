package testcode

import (
	"net/http"
)

func BenchmarkTest00024(w http.ResponseWriter, r *http.Request) {
	_ = r.URL.Query().Get("id")
	defaultID := "1"
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", defaultID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

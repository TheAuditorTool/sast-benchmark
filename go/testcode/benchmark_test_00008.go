package testcode

import (
	"net/http"
)

func BenchmarkTest00008(w http.ResponseWriter, r *http.Request) {
	rows, err := DB.Query("SELECT COUNT(*) FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

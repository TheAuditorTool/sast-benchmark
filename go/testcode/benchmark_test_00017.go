package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00017(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_data")
	if err != nil {
		http.Error(w, "missing cookie", http.StatusBadRequest)
		return
	}
	query := fmt.Sprintf("SELECT * FROM sessions WHERE token = '%s'", cookie.Value)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

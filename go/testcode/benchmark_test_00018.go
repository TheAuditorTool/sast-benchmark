package testcode

import (
	"net/http"
)

func BenchmarkTest00018(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_data")
	if err != nil {
		http.Error(w, "missing cookie", http.StatusBadRequest)
		return
	}
	rows, err := DB.Query("SELECT * FROM sessions WHERE token = ?", cookie.Value)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

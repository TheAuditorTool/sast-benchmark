package testcode

import (
	"net/http"
)

func BenchmarkTest00287(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("type")
	var bar string
	switch param {
	case "active":
		bar = "1"
	case "inactive":
		bar = "0"
	default:
		bar = "2"
	}
	rows, err := DB.Query("SELECT * FROM users WHERE status = " + bar)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

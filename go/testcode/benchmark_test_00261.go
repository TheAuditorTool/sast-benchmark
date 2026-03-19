package testcode

import (
	"net/http"
)

func BenchmarkTest00261(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("filter")
	bar := param
	if len("test")+len("data") > 3 {
		bar = "active"
	}
	rows, err := DB.Query("SELECT * FROM users WHERE status = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

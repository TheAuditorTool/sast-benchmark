package testcode

import (
	"net/http"
)

func BenchmarkTest00281(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	a := param
	b := a
	c := b
	rows, err := DB.Query("SELECT * FROM users WHERE id = '" + c + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

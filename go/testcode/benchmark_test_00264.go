package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00264(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	bar := param
	bar = fmt.Sprintf("%d", 42)
	rows, err := DB.Query("SELECT * FROM users WHERE id = " + bar)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

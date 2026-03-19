package testcode

import (
	"net/http"
)

type benchmarkTest00283Request struct {
	ID string
}

func BenchmarkTest00283(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	req := benchmarkTest00283Request{ID: param}
	req.ID = "1"
	rows, err := DB.Query("SELECT * FROM users WHERE id = " + req.ID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

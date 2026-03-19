package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00267(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("data")
	bar := param
	bar = strconv.Itoa(len(param))
	rows, err := DB.Query("SELECT * FROM analytics WHERE input_length = " + bar)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

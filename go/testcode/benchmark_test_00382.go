package testcode

import (
	"net/http"
)

func BenchmarkTest00382(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	items := r.Form["items"]
	stmt, err := DB.Prepare("INSERT INTO cart (name) VALUES (?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()
	for _, item := range items {
		stmt.Exec(item)
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "added"})
}

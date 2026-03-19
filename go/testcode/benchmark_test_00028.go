package testcode

import (
	"net/http"
)

func BenchmarkTest00028(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	items := r.Form["item"]
	stmt, err := DB.Prepare("INSERT INTO cart (item_name) VALUES (?)")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()
	for _, item := range items {
		stmt.Exec(item)
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "added"})
}

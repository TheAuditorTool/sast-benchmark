package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00381(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	items := r.Form["items"]
	var values string
	for _, item := range items {
		values += "('" + item + "'),"
	}
	DB.Exec("INSERT INTO cart (name) VALUES " + strings.TrimRight(values, ","))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "added"})
}

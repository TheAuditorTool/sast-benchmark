package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00027(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	items := r.Form["item"]
	for _, item := range items {
		query := fmt.Sprintf("INSERT INTO cart (item_name) VALUES ('%s')", item)
		DB.Exec(query)
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "added"})
}

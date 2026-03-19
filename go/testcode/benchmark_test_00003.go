package testcode

import (
	"net/http"
)

func BenchmarkTest00003(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	query := "SELECT * FROM users WHERE username = '" + username + "'"
	row := DB.QueryRow(query)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)
	RespondJSON(w, http.StatusOK, map[string]string{"username": name, "email": email})
}

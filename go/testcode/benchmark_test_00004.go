package testcode

import (
	"net/http"
)

func BenchmarkTest00004(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	stmt, err := DB.Prepare("SELECT * FROM users WHERE username = ?")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer stmt.Close()
	row := stmt.QueryRow(username)
	var id int
	var name, email string
	_ = row.Scan(&id, &name, &email)
	RespondJSON(w, http.StatusOK, map[string]string{"username": name, "email": email})
}

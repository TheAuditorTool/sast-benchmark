package testcode

import (
	"net/http"
)

type SqlxUser struct {
	ID   int    `db:"id"`
	Name string `db:"name"`
}

func BenchmarkTest00378(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var user SqlxUser
	err := SqlxDB.Get(&user, "SELECT * FROM users WHERE id = $1", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, user)
}

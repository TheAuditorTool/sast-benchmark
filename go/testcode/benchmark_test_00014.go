package testcode

import (
	"net/http"
)

func BenchmarkTest00014(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	email := r.PostFormValue("email")
	role := r.PostFormValue("role")
	id := r.URL.Query().Get("id")
	_, err := DB.Exec("UPDATE users SET email = ?, role = ? WHERE id = ?", email, role, id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

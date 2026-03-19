package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00013(w http.ResponseWriter, r *http.Request) {
	r.ParseForm()
	email := r.PostFormValue("email")
	role := r.PostFormValue("role")
	id := r.URL.Query().Get("id")
	query := fmt.Sprintf(
		"UPDATE users SET email = '%s', role = '%s' WHERE id = %s",
		email, role, id,
	)
	_, err := DB.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

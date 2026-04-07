package testcode

import (
	"net/http"
)

func BenchmarkTest01143(w http.ResponseWriter, r *http.Request) {
	rows, err := DB.Query("SELECT id, username, email FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var users []map[string]string
	for rows.Next() {
		var id, username, email string
		rows.Scan(&id, &username, &email)
		users = append(users, map[string]string{"id": id, "username": username, "email": email})
	}
	RespondJSON(w, http.StatusOK, users)
}

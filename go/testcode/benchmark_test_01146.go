package testcode

import (
	"net/http"
)

func BenchmarkTest01146(w http.ResponseWriter, r *http.Request) {
	rows, err := DB.Query("SELECT id, username, email, phone, address FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	var records []map[string]string
	for rows.Next() {
		var id, username, email, phone, address string
		rows.Scan(&id, &username, &email, &phone, &address)
		records = append(records, map[string]string{
			"id": id, "username": username, "email": email,
			"phone": phone, "address": address,
		})
	}
	RespondJSON(w, http.StatusOK, records)
}

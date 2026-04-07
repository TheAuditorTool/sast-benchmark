package testcode

import (
	"net/http"

	"github.com/jmoiron/sqlx"
)

var benchmarkTest00692DB *sqlx.DB

func BenchmarkTest00692(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	status := r.URL.Query().Get("status")

	rows, err := benchmarkTest00692DB.NamedQuery(
		"SELECT u.id, u.name FROM users u JOIN orders o ON u.id = o.user_id WHERE u.username = :username AND o.status = :status",
		map[string]interface{}{"username": username, "status": status},
	)
	if err != nil {
		http.Error(w, "query error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	type result struct {
		ID   int    `db:"id"`
		Name string `db:"name"`
	}
	var results []result
	for rows.Next() {
		var res result
		if err := rows.StructScan(&res); err != nil {
			http.Error(w, "scan error", http.StatusInternalServerError)
			return
		}
		results = append(results, res)
	}
	RespondJSON(w, http.StatusOK, results)
}

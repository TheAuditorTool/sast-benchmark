package testcode

import (
	"net/http"
)

func BenchmarkTest00611(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	tableName := r.URL.Query().Get("table_name")

	var query string
	switch tableName {
	case "users":
		query = "SELECT * FROM users"
	case "orders":
		query = "SELECT * FROM orders"
	case "payments":
		query = "SELECT * FROM payments"
	case "audit_logs":
		query = "SELECT * FROM audit_logs"
	default:
		http.Error(w, "unknown table", http.StatusBadRequest)
		return
	}

	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, "export failed", http.StatusInternalServerError)
		return
	}
	defer rows.Close()

	cols, err := rows.Columns()
	if err != nil {
		http.Error(w, "column fetch failed", http.StatusInternalServerError)
		return
	}

	var results []map[string]interface{}
	for rows.Next() {
		vals := make([]interface{}, len(cols))
		ptrs := make([]interface{}, len(cols))
		for i := range vals {
			ptrs[i] = &vals[i]
		}
		if err := rows.Scan(ptrs...); err != nil {
			continue
		}
		row := make(map[string]interface{}, len(cols))
		for i, col := range cols {
			row[col] = vals[i]
		}
		results = append(results, row)
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"table":   tableName,
		"records": results,
	})
}

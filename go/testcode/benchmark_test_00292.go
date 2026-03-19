package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00292(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("name")
	formatted := fmt.Sprintf("user_%s", param)
	rows, err := DB.Query("SELECT * FROM users WHERE name = '" + formatted + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

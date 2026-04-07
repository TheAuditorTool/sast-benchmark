package testcode

import (
	"math/rand"
	"net/http"
)

func BenchmarkTest00781(w http.ResponseWriter, r *http.Request) {
	offset := rand.Intn(5)
	rows, err := DB.Query("SELECT id, name FROM products LIMIT 10 OFFSET ?", offset)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

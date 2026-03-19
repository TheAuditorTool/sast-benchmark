package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00047(w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	parsedID, err := uuid.Parse(idStr)
	if err != nil {
		http.Error(w, "invalid uuid", http.StatusBadRequest)
		return
	}
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", parsedID.String())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

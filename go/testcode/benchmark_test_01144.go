package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01144(w http.ResponseWriter, r *http.Request) {
	itemID := strings.TrimPrefix(r.URL.Path, "/items/")
	if itemID == "" {
		http.Error(w, "missing item id", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec("DELETE FROM items WHERE id = ?", itemID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}

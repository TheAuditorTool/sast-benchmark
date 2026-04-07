package testcode

import (
	"net/http"
)

func BenchmarkTest01313(w http.ResponseWriter, r *http.Request) {
	order := r.URL.Query().Get("order")
	allowed := map[string]bool{"asc": true, "desc": true}
	if !allowed[order] {
		http.Error(w, "invalid order", http.StatusBadRequest)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"order": order})
}

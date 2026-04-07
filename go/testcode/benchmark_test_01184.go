package testcode

import (
	"net/http"
)

func BenchmarkTest01184(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var body struct {
		ResourceID string `json:"resource_id"`
		Value      string `json:"value"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid json", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("UPDATE resources SET value = ? WHERE id = ?", body.Value, body.ResourceID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

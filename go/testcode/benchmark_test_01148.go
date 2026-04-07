package testcode

import (
	"net/http"
)

func benchmarkTest01148PerformWrite(targetID string, data map[string]interface{}) error {
	_, err := DB.Exec("UPDATE accounts SET balance = ? WHERE id = ?", data["balance"], targetID)
	return err
}

func BenchmarkTest01148(w http.ResponseWriter, r *http.Request) {
	role := r.Header.Get("X-User-Role")
	if role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var body map[string]interface{}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	targetID, _ := body["target_id"].(string)
	if err := benchmarkTest01148PerformWrite(targetID, body); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

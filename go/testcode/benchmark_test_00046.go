package testcode

import (
	"encoding/json"
	"fmt"
	"net/http"
)

func BenchmarkTest00046(w http.ResponseWriter, r *http.Request) {
	var raw interface{}
	json.NewDecoder(r.Body).Decode(&raw)
	if m, ok := raw.(map[string]interface{}); ok {
		if name, ok := m["name"].(string); ok {
			query := fmt.Sprintf("SELECT * FROM users WHERE name = '%s'", name)
			DB.Query(query)
		}
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

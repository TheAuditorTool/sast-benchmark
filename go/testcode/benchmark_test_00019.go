package testcode

import (
	"encoding/json"
	"fmt"
	"net/http"
)

func BenchmarkTest00019(w http.ResponseWriter, r *http.Request) {
	var body map[string]string
	json.NewDecoder(r.Body).Decode(&body)
	for field, value := range body {
		query := fmt.Sprintf("UPDATE settings SET %s = '%s' WHERE id = 1", field, value)
		DB.Exec(query)
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

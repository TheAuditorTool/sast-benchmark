package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00385(w http.ResponseWriter, r *http.Request) {
	userInput := r.FormValue("action")
	query := fmt.Sprintf("INSERT INTO audit (action) VALUES ('%s')", userInput)
	defer func() {
		DB.Exec(query)
	}()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

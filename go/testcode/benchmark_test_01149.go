package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01149(w http.ResponseWriter, r *http.Request) {
	resourceID := strings.TrimPrefix(r.URL.Path, "/resources/")
	sessionToken := r.Header.Get("X-Session-Token")

	authResult := make(chan bool, 1)
	go func() {
		var role string
		err := DB.QueryRow("SELECT role FROM sessions WHERE token = ?", sessionToken).Scan(&role)
		authResult <- (err == nil && role == "admin")
	}()

	_, err := DB.Exec("DELETE FROM resources WHERE id = ?", resourceID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if !<-authResult {
		RespondJSON(w, http.StatusOK, map[string]string{"note": "auth checked after operation"})
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "deleted"})
}

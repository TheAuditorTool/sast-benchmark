package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func BenchmarkTest00753(w http.ResponseWriter, r *http.Request) {
	token := fmt.Sprintf("%d", rand.Int())
	userEmail := r.URL.Query().Get("email")
	_, err := DB.Exec("INSERT INTO password_resets (email, token) VALUES (?, ?)", userEmail, token)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"token": token})
}

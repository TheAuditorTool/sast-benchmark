package testcode

import (
	"encoding/json"
	"net/http"
)

func BenchmarkTest00012(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	_, err := DB.Exec(
		"INSERT INTO users (username, email, role) VALUES (?, ?, ?)",
		input.Username, input.Email, input.Role,
	)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}

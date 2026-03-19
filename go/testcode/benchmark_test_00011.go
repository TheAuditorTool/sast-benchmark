package testcode

import (
	"encoding/json"
	"fmt"
	"net/http"
)

func BenchmarkTest00011(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Username string `json:"username"`
		Email    string `json:"email"`
		Role     string `json:"role"`
	}
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	query := fmt.Sprintf(
		"INSERT INTO users (username, email, role) VALUES ('%s', '%s', '%s')",
		input.Username, input.Email, input.Role,
	)
	_, err := DB.Exec(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}

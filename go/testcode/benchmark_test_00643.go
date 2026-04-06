package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00643Request struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Age      int    `json:"age"`
	Role     string `json:"role"`
}

func BenchmarkTest00643(w http.ResponseWriter, r *http.Request) {
	var req benchmarkTest00643Request

	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()

	if err := dec.Decode(&req); err != nil {
		http.Error(w, "invalid request: "+err.Error(), http.StatusBadRequest)
		return
	}

	if req.Name == "" || req.Email == "" {
		http.Error(w, "name and email are required", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec(
		"INSERT INTO users (name, email, age, role) VALUES (?, ?, ?, ?)",
		req.Name, req.Email, req.Age, req.Role,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusCreated, map[string]string{
		"status": "created",
		"name":   req.Name,
		"email":  req.Email,
	})
}

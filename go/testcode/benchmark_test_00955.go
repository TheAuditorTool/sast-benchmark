package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00955Profile struct {
	UserID   int    `json:"user_id"`
	Username string `json:"username"`
	Bio      string `json:"bio"`
	Age      int    `json:"age"`
}

func BenchmarkTest00955(w http.ResponseWriter, r *http.Request) {
	var p benchmarkTest00955Profile
	if err := json.NewDecoder(r.Body).Decode(&p); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("INSERT INTO profiles (user_id, username, bio, age) VALUES (?, ?, ?, ?)",
		p.UserID, p.Username, p.Bio, p.Age)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "created"})
}

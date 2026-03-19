package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest00165(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	hashed, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		http.Error(w, "hashing failed", http.StatusInternalServerError)
		return
	}

	_, err = DB.Exec("INSERT INTO users (username, password_hash) VALUES (?, ?)",
		req.Username, string(hashed))
	if err != nil {
		http.Error(w, "user creation failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": "user registered",
	})
}

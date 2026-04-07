package testcode

import (
	"net/http"

	"golang.org/x/crypto/bcrypt"
)

func BenchmarkTest01134(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := ParseJSONBody(r, &req); err != nil || req.Username == "" || req.Password == "" {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	hashed, err := bcrypt.GenerateFromPassword([]byte(req.Password), bcrypt.DefaultCost)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	_, err = DB.Exec("INSERT INTO users (username, password) VALUES (?, ?)", req.Username, string(hashed))
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusCreated, map[string]string{"status": "registered"})
}

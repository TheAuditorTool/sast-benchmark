package testcode

import (
	"net/http"
)

type benchmarkTest01344UserResponse struct {
	Name     string `json:"name"`
	Email    string `json:"email"`
	Password string `json:"-"`
	APIKey   string `json:"-"`
}

func BenchmarkTest01344(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var u benchmarkTest01344UserResponse
	err := DB.QueryRow("SELECT name, email, password, api_key FROM users WHERE id = ?", id).
		Scan(&u.Name, &u.Email, &u.Password, &u.APIKey)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, u)
}

package testcode

import (
	"net/http"
)

type benchmarkTest00680AppError struct {
	internal string
	code     int
}

func (e *benchmarkTest00680AppError) Error() string {
	return "an error occurred"
}

func benchmarkTest00680QueryUser(id string) (string, error) {
	var email string
	err := DB.QueryRow("SELECT email FROM users WHERE id = ?", id).Scan(&email)
	if err != nil {
		return "", &benchmarkTest00680AppError{
			internal: "users table lookup failed for id=" + id + ": " + err.Error(),
			code:     500,
		}
	}
	return email, nil
}

func BenchmarkTest00680(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	if id == "" {
		http.Error(w, "id required", http.StatusBadRequest)
		return
	}

	email, err := benchmarkTest00680QueryUser(id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"email": email})
}

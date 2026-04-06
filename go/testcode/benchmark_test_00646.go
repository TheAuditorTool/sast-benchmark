package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00646(w http.ResponseWriter, r *http.Request) {
	if err := r.ParseForm(); err != nil {
		http.Error(w, "invalid form data", http.StatusBadRequest)
		return
	}

	name := strings.TrimSpace(r.FormValue("name"))
	if len(name) == 0 || len(name) > 100 {
		http.Error(w, "name must be between 1 and 100 characters", http.StatusBadRequest)
		return
	}

	description := strings.TrimSpace(r.FormValue("description"))
	if len(description) > 500 {
		http.Error(w, "description must not exceed 500 characters", http.StatusBadRequest)
		return
	}

	_, err := DB.Exec(
		"INSERT INTO categories (name, description) VALUES (?, ?)",
		name, description,
	)
	if err != nil {
		http.Error(w, "database error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusCreated, map[string]string{
		"status": "created",
		"name":   name,
	})
}

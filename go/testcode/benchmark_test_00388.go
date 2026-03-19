package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest00388(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20)
	_, _, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	safeName := uuid.New().String()
	DB.Exec("INSERT INTO files (name) VALUES (?)", safeName)
	RespondJSON(w, http.StatusOK, map[string]string{"name": safeName})
}

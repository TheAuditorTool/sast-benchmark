package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00387(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(10 << 20)
	_, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	DB.Exec(fmt.Sprintf("INSERT INTO files (name) VALUES ('%s')", header.Filename))
	RespondJSON(w, http.StatusOK, map[string]string{"status": "uploaded"})
}

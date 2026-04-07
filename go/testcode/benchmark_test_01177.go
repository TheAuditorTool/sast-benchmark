package testcode

import (
	"net/http"
)

func BenchmarkTest01177(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	err := r.ParseMultipartForm(10 << 20)
	if err != nil {
		http.Error(w, "parse error", http.StatusBadRequest)
		return
	}
	file, header, err := r.FormFile("upload")
	if err != nil {
		http.Error(w, "missing file", http.StatusBadRequest)
		return
	}
	defer file.Close()
	userID := r.FormValue("user_id")
	_, err = DB.Exec("INSERT INTO uploads (user_id, filename, size) VALUES (?, ?, ?)", userID, header.Filename, header.Size)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "uploaded"})
}

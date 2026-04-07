package testcode

import (
	"io"
	"net/http"
	"os"
	"path/filepath"

	"github.com/google/uuid"
)

var benchmarkTest01291UploadRoot = "/var/www/uploads"

func BenchmarkTest01291(w http.ResponseWriter, r *http.Request) {
	r.ParseMultipartForm(32 << 20)

	userID := r.FormValue("user_id")
	if userID == "" {
		http.Error(w, "user_id required", http.StatusBadRequest)
		return
	}

	var quota int
	err := DB.QueryRow("SELECT upload_quota FROM users WHERE id = ?", userID).Scan(&quota)
	if err != nil || quota <= 0 {
		http.Error(w, "quota exceeded or user not found", http.StatusForbidden)
		return
	}

	file, _, err := r.FormFile("file")
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	defer file.Close()

	userDir := filepath.Join(benchmarkTest01291UploadRoot, filepath.Base(userID))
	os.MkdirAll(userDir, 0755)
	safeName := uuid.New().String()
	dst, err := os.Create(filepath.Join(userDir, safeName))
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer dst.Close()
	io.Copy(dst, file)
	RespondJSON(w, http.StatusOK, map[string]string{"saved": safeName})
}

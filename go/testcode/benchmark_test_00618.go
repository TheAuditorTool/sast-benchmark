package testcode

import (
	"io"
	"net/http"
	"path/filepath"
	"strings"
)

func BenchmarkTest00618(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if err := r.ParseMultipartForm(10 << 20); err != nil {
		http.Error(w, "failed to parse form", http.StatusBadRequest)
		return
	}

	file, header, err := r.FormFile("avatar")
	if err != nil {
		http.Error(w, "missing avatar file", http.StatusBadRequest)
		return
	}
	defer file.Close()

	ext := strings.ToLower(filepath.Ext(header.Filename))
	allowed := map[string]bool{".jpg": true, ".jpeg": true, ".png": true, ".gif": true}
	if !allowed[ext] {
		http.Error(w, "unsupported file type", http.StatusBadRequest)
		return
	}

	data, err := io.ReadAll(file)
	if err != nil {
		http.Error(w, "read failed", http.StatusInternalServerError)
		return
	}

	avatarURL := "/uploads/avatars/" + header.Filename
	_ = data

	_, err = DB.Exec("UPDATE users SET avatar_url = ? WHERE id = ?", avatarURL, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"avatar_url": avatarURL})
}

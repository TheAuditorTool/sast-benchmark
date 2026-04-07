package testcode

import (
	"crypto/md5"
	"encoding/base64"
	"net/http"
)

func BenchmarkTest00793(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	raw := md5.Sum([]byte(password))
	encoded := base64.StdEncoding.EncodeToString(raw[:])
	_, err := DB.Exec("INSERT INTO users (username, pw) VALUES (?, ?)", username, encoded)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}

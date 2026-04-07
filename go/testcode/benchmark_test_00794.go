package testcode

import (
	"crypto/md5"
	"crypto/sha1"
	"fmt"
	"net/http"
)

func BenchmarkTest00794(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	s1 := sha1.Sum([]byte(password))
	m := md5.Sum(s1[:])
	hashStr := fmt.Sprintf("%x", m)
	_, err := DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}

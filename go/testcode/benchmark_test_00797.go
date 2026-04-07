package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

func BenchmarkTest00797(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	username := r.FormValue("username")
	pepper := r.FormValue("pepper")
	h := md5.Sum([]byte(password + pepper))
	hashStr := fmt.Sprintf("%x", h)
	_, err := DB.Exec("INSERT INTO users (username, pw_hash) VALUES (?, ?)", username, hashStr)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}

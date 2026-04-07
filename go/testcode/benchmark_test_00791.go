package testcode

import (
	"crypto/sha1"
	"fmt"
	"net/http"
)

type benchmarkTest00791Account struct {
	Username string
	PwHash   string
}

func BenchmarkTest00791(w http.ResponseWriter, r *http.Request) {
	pw := r.FormValue("password")
	h := sha1.Sum([]byte(pw))
	acc := benchmarkTest00791Account{
		Username: r.FormValue("username"),
		PwHash:   fmt.Sprintf("%x", h),
	}
	_, err := DB.Exec("INSERT INTO accounts (username, pw_hash) VALUES (?, ?)", acc.Username, acc.PwHash)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}

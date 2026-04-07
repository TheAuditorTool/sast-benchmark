package testcode

import (
	"crypto/md5"
	"fmt"
	"net/http"
)

type benchmarkTest00789User struct {
	Username string
	Password string
}

func (u benchmarkTest00789User) HashPassword() string {
	h := md5.Sum([]byte(u.Password))
	return fmt.Sprintf("%x", h)
}

func BenchmarkTest00789(w http.ResponseWriter, r *http.Request) {
	u := benchmarkTest00789User{
		Username: r.FormValue("username"),
		Password: r.FormValue("password"),
	}
	hash := u.HashPassword()
	_, err := DB.Exec("INSERT INTO accounts (username, pw_hash) VALUES (?, ?)", u.Username, hash)
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusCreated, map[string]string{"status": "ok"})
}

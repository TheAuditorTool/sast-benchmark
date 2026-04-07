package testcode

import (
	"context"
	"crypto/md5"
	"fmt"
	"net/http"
)

type benchmarkTest00792KeyType struct{}

func BenchmarkTest00792(w http.ResponseWriter, r *http.Request) {
	password := r.FormValue("password")
	ctx := context.WithValue(r.Context(), benchmarkTest00792KeyType{}, password)
	pw := ctx.Value(benchmarkTest00792KeyType{}).(string)
	h := md5.Sum([]byte(pw))
	hashStr := fmt.Sprintf("%x", h)
	_, err := DB.Exec("UPDATE users SET pw_hash = ? WHERE username = ?", hashStr, r.FormValue("username"))
	if err != nil {
		http.Error(w, "db error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}

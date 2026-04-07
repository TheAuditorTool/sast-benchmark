package testcode

import (
	"encoding/json"
	"net/http"
)

type benchmarkTest00962LoginReq struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func BenchmarkTest00962(w http.ResponseWriter, r *http.Request) {
	r.Body = http.MaxBytesReader(w, r.Body, 4096)
	dec := json.NewDecoder(r.Body)
	dec.DisallowUnknownFields()
	var req benchmarkTest00962LoginReq
	if err := dec.Decode(&req); err != nil {
		http.Error(w, "json error", http.StatusBadRequest)
		return
	}
	var stored string
	err := DB.QueryRow("SELECT password_hash FROM users WHERE username = ?", req.Username).Scan(&stored)
	if err != nil || stored != req.Password {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

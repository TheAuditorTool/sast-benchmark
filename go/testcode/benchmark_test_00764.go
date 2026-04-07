package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

func BenchmarkTest00764(w http.ResponseWriter, r *http.Request) {
	src := rand.NewSource(time.Now().UnixNano())
	rng := rand.New(src)
	token := fmt.Sprintf("%x", rng.Int63())
	userID := r.URL.Query().Get("user_id")
	_, _ = DB.Exec("UPDATE users SET api_key = ? WHERE id = ?", token, userID)
	RespondJSON(w, http.StatusOK, map[string]string{"api_key": token})
}

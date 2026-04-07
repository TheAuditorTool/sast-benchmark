package testcode

import (
	"net/http"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

func BenchmarkTest01066(w http.ResponseWriter, r *http.Request) {
	tokenStr := strings.TrimPrefix(r.Header.Get("Authorization"), "Bearer ")
	token, _, _ := jwt.NewParser().ParseUnverified(tokenStr, jwt.MapClaims{})
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	if token != nil {
		claims, _ := token.Claims.(jwt.MapClaims)
		sess.Values["groups"] = claims["groups"]
	}
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

package testcode

import (
	"context"
	"net/http"
)

type benchmarkTest01061UserCtxKey struct{}

type benchmarkTest01061User struct {
	ID    string
	Role  string
	Admin bool
}

func BenchmarkTest01061(w http.ResponseWriter, r *http.Request) {
	user := benchmarkTest01061User{
		ID:    r.FormValue("user_id"),
		Role:  r.FormValue("role"),
		Admin: r.FormValue("admin") == "true",
	}
	ctx := context.WithValue(r.Context(), benchmarkTest01061UserCtxKey{}, user)
	u := ctx.Value(benchmarkTest01061UserCtxKey{}).(benchmarkTest01061User)
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["user_id"] = u.ID
	sess.Values["role"] = u.Role
	sess.Values["admin"] = u.Admin
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

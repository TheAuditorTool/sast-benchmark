package testcode

import "net/http"

type benchmarkTest01058UserInfo struct {
	UserID  string
	Role    string
	IsAdmin string
}

func BenchmarkTest01058(w http.ResponseWriter, r *http.Request) {
	info := benchmarkTest01058UserInfo{
		UserID:  r.FormValue("user_id"),
		Role:    r.FormValue("role"),
		IsAdmin: r.FormValue("is_admin"),
	}
	sess, err := SessionStore.Get(r, "session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	sess.Values["user_id"] = info.UserID
	sess.Values["role"] = info.Role
	sess.Values["is_admin"] = info.IsAdmin
	SessionStore.Save(r, w, sess)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

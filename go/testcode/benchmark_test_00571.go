package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"net/url"
	"os"
	"strings"
)

func BenchmarkTest00571(w http.ResponseWriter, r *http.Request) {
	accessToken := r.Header.Get("Authorization")
	if accessToken == "" {
		http.Error(w, "missing token", http.StatusUnauthorized)
		return
	}

	authServer := os.Getenv("AUTH_SERVER")
	form := url.Values{}
	form.Set("token", strings.TrimPrefix(accessToken, "Bearer "))

	resp, err := http.Post(authServer+"/introspect", "application/x-www-form-urlencoded", strings.NewReader(form.Encode()))
	if err != nil {
		http.Error(w, "introspection failed", http.StatusServiceUnavailable)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var introspect struct {
		Active bool   `json:"active"`
		Scope  string `json:"scope"`
	}
	if err := json.Unmarshal(body, &introspect); err != nil || !introspect.Active {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	session, err := SessionStore.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}

	session.Values["api_scope"] = introspect.Scope

	if err := session.Save(r, w); err != nil {
		http.Error(w, "session save error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

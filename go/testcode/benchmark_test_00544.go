package testcode

import (
	"net/http"

	"github.com/gorilla/securecookie"
)

var (
	benchmarkTest00544Current  = securecookie.New([]byte("current-hmac-key-32-bytes-padded!"), []byte("current-enc-key-32-bytes-padded!!"))
	benchmarkTest00544Previous = securecookie.New([]byte("old-hmac-key-32-bytes-padded-here"), []byte("old-enc-key-32-bytes-padded-here!"))
)

func BenchmarkTest00544(w http.ResponseWriter, r *http.Request) {
	var body struct {
		UserID string `json:"user_id"`
		Role   string `json:"role"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	sessionData := map[string]string{
		"user_id": body.UserID,
		"role":    body.Role,
	}

	encoded, err := benchmarkTest00544Current.Encode("session", sessionData)
	if err != nil {
		http.Error(w, "session encode error", http.StatusInternalServerError)
		return
	}

	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    encoded,
		Secure:   true,
		HttpOnly: true,
		SameSite: http.SameSiteStrictMode,
		Path:     "/",
		MaxAge:   86400,
	})

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

func benchmarkTest00544DecodeSession(r *http.Request) (map[string]string, error) {
	cookie, err := r.Cookie("session")
	if err != nil {
		return nil, err
	}

	codecs := securecookie.CodecsFromPairs(
		[]byte("current-hmac-key-32-bytes-padded!"), []byte("current-enc-key-32-bytes-padded!!"),
		[]byte("old-hmac-key-32-bytes-padded-here"), []byte("old-enc-key-32-bytes-padded-here!"),
	)

	var data map[string]string
	for _, codec := range codecs {
		if err := codec.Decode("session", cookie.Value, &data); err == nil {
			return data, nil
		}
	}
	return nil, http.ErrNoCookie
}

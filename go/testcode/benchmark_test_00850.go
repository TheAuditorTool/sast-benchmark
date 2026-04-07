package testcode

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"net/http"
)

type benchmarkTest00850CtxKey struct{}

type benchmarkTest00850Opts struct {
	HttpOnly bool
	Secure   bool
}

func BenchmarkTest00850(w http.ResponseWriter, r *http.Request) {
	opts := benchmarkTest00850Opts{HttpOnly: true, Secure: false}
	ctx := context.WithValue(r.Context(), benchmarkTest00850CtxKey{}, opts)
	o := ctx.Value(benchmarkTest00850CtxKey{}).(benchmarkTest00850Opts)
	b := make([]byte, 16)
	rand.Read(b)
	http.SetCookie(w, &http.Cookie{
		Name:     "session",
		Value:    hex.EncodeToString(b),
		HttpOnly: o.HttpOnly,
		Secure:   o.Secure,
	})
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

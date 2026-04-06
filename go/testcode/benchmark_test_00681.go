package testcode

import (
	"net/http"
	"net/http/pprof"
)

func benchmarkTest00681BasicAuth(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		user, pass, ok := r.BasicAuth()
		if !ok || user != "admin" || pass != "debug-only-2024!" {
			w.Header().Set("WWW-Authenticate", `Basic realm="debug"`)
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}
		next.ServeHTTP(w, r)
	})
}

var benchmarkTest00681DebugMux = func() *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("/debug/pprof/", pprof.Index)
	mux.HandleFunc("/debug/pprof/cmdline", pprof.Cmdline)
	mux.HandleFunc("/debug/pprof/profile", pprof.Profile)
	mux.HandleFunc("/debug/pprof/symbol", pprof.Symbol)
	mux.HandleFunc("/debug/pprof/trace", pprof.Trace)
	return mux
}()

func BenchmarkTest00681(w http.ResponseWriter, r *http.Request) {
	if len(r.URL.Path) >= 12 && r.URL.Path[:12] == "/debug/pprof" {
		benchmarkTest00681BasicAuth(benchmarkTest00681DebugMux).ServeHTTP(w, r)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

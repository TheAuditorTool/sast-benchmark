package testcode

import (
	"net/http"
	"os"
)

type benchmarkTest01108Env struct {
	DBPass    string
	RedisPass string
	APISecret string
}

func benchmarkTest01108LoadEnv() benchmarkTest01108Env {
	return benchmarkTest01108Env{
		DBPass:    os.Getenv("DB_PASS"),
		RedisPass: os.Getenv("REDIS_PASS"),
		APISecret: os.Getenv("API_SECRET"),
	}
}

func BenchmarkTest01108(w http.ResponseWriter, r *http.Request) {
	env := benchmarkTest01108LoadEnv()
	RespondJSON(w, http.StatusOK, map[string]bool{
		"ready": env.DBPass != "" && env.APISecret != "",
	})
}

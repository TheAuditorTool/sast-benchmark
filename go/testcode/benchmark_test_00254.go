package testcode

import (
	"io"
	"net/http"

	"gopkg.in/yaml.v3"
)

type appConfig struct {
	Database struct {
		Host     string `yaml:"host"`
		Port     int    `yaml:"port"`
		Name     string `yaml:"name"`
		MaxConns int    `yaml:"max_conns"`
	} `yaml:"database"`
	Server struct {
		Port    int    `yaml:"port"`
		Address string `yaml:"address"`
	} `yaml:"server"`
	LogLevel string `yaml:"log_level"`
}

func BenchmarkTest00254(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var config appConfig
	err = yaml.Unmarshal(body, &config)
	if err != nil {
		http.Error(w, "yaml parse error", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec("INSERT INTO configs (db_host, db_port, db_name, server_port, log_level) VALUES (?, ?, ?, ?, ?)",
		config.Database.Host, config.Database.Port, config.Database.Name,
		config.Server.Port, config.LogLevel)
	if err != nil {
		http.Error(w, "storage error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message":  "config saved",
		"log_level": config.LogLevel,
	})
}

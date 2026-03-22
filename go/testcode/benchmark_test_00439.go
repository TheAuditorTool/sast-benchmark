package testcode

import (
	"net/http"
	"os"

	"golang.org/x/crypto/ssh"
)

func BenchmarkTest00439(w http.ResponseWriter, r *http.Request) {
	keyPath := os.Getenv("SSH_KEY_PATH")
	if keyPath == "" {
		http.Error(w, "service not configured", http.StatusInternalServerError)
		return
	}

	keyBytes, err := os.ReadFile(keyPath)
	if err != nil {
		http.Error(w, "key unavailable", http.StatusInternalServerError)
		return
	}

	signer, err := ssh.ParsePrivateKey(keyBytes)
	if err != nil {
		http.Error(w, "key initialization failed", http.StatusInternalServerError)
		return
	}

	host := r.URL.Query().Get("host")
	if host == "" {
		http.Error(w, "missing host parameter", http.StatusBadRequest)
		return
	}

	config := &ssh.ClientConfig{
		User: "deploy",
		Auth: []ssh.AuthMethod{
			ssh.PublicKeys(signer),
		},
		HostKeyCallback: ssh.InsecureIgnoreHostKey(),
	}

	conn, err := ssh.Dial("tcp", host+":22", config)
	if err != nil {
		http.Error(w, "ssh connection failed", http.StatusBadGateway)
		return
	}
	defer conn.Close()

	RespondJSON(w, http.StatusOK, map[string]string{"host": host, "status": "connected"})
}

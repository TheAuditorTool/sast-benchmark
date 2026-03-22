package testcode

import (
	"net/http"

	"golang.org/x/crypto/ssh"
)

const benchmarkTest00433SSHKey = "-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA2a2rwplBQLzHPZe5TNJNE4K4FlMNAFIt4+RfurNkSCMRhzMa\nA1a8BEnUHIBQAz/YCBJWbEcOyDEfCQnHkCqrOBHqplATFNMoI3YxSvtGEXdlAVsJ\nqdCXNrFkPnKGCLTWDkGKBtTMRF1fZSLRb0wbVx0qPFG3vLn0tTVKEYv0jE1aFdXA\nkBLp3jOIOrFi2q5Xo+JDvPHfI7e6ZiJZCLNkSMrFp9dKn8Yv6eNvBrHaF5vLmIQZ\nTEbU4r8X1kN3AMoP+JHqW2GN2DjnP3K+vLpA2wMRbzuJBdBgm1tRW1YcC9k6P+lB\nHAI6x8mMWrFn7f/LzP8tTQ5pnz3e6d+K+rGlmQIDAQABAoIBAHJtReHkdA1RFQB1\nmP1dA09N1xCXPSMT5M5T2k3cMn9h7dAtWGPRUBZHjF6v8dkTqVHGECn9mhFHuBcW\nkSPvKQ0Z8xkTVnQ7xOd1n7e0lHMJ8kT4G0b5Av8pS7rExBkWOZgCKkgfNmZ8dHJX\n-----END RSA PRIVATE KEY-----"

func BenchmarkTest00433(w http.ResponseWriter, r *http.Request) {
	signer, err := ssh.ParsePrivateKey([]byte(benchmarkTest00433SSHKey))
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

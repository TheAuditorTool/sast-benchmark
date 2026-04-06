package testcode

import (
	"encoding/json"
	"io"
	"net/http"
	"net/url"
	"strings"
)

func BenchmarkTest00549(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Code string `json:"code"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	clientSecret := "sk_live_4eC39HqLyjWDarjtT1zdp7dc"

	form := url.Values{}
	form.Set("client_id", "oauth_client_prod_001")
	form.Set("client_secret", clientSecret)
	form.Set("code", req.Code)
	form.Set("grant_type", "authorization_code")

	resp, err := http.Post(
		"https://auth.provider.example.com/oauth/token",
		"application/x-www-form-urlencoded",
		strings.NewReader(form.Encode()),
	)
	if err != nil {
		http.Error(w, "upstream error", http.StatusBadGateway)
		return
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	var tokenResp map[string]interface{}
	if err := json.Unmarshal(body, &tokenResp); err != nil {
		http.Error(w, "invalid upstream response", http.StatusBadGateway)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"access_token": tokenResp["access_token"],
		"token_type":   tokenResp["token_type"],
	})
}

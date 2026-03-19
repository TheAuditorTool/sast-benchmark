package testcode

import (
	"encoding/json"
	"io"
	"net/http"
)

type userProfile struct {
	Name    string `json:"name"`
	Email   string `json:"email"`
	Age     int    `json:"age"`
	Country string `json:"country"`
}

func BenchmarkTest00253(w http.ResponseWriter, r *http.Request) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "read error", http.StatusBadRequest)
		return
	}

	var profile userProfile
	err = json.Unmarshal(body, &profile)
	if err != nil {
		http.Error(w, "json parse error", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec("INSERT INTO profiles (name, email, age, country) VALUES (?, ?, ?, ?)",
		profile.Name, profile.Email, profile.Age, profile.Country)
	if err != nil {
		http.Error(w, "storage error", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"message": "profile created",
		"name":    profile.Name,
	})
}

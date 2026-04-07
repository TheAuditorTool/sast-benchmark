package testcode

import (
	"net/http"
	"net/mail"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01045Conn *ldap.Conn

func BenchmarkTest01045(w http.ResponseWriter, r *http.Request) {
	email := r.URL.Query().Get("email")
	if _, err := mail.ParseAddress(email); err != nil {
		http.Error(w, "invalid email", http.StatusBadRequest)
		return
	}
	filter := "(mail=" + ldap.EscapeFilter(email) + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn"}, nil,
	)
	result, err := benchmarkTest01045Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

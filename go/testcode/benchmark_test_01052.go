package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01052Conn *ldap.Conn

func BenchmarkTest01052(w http.ResponseWriter, r *http.Request) {
	username := ldap.EscapeFilter(r.URL.Query().Get("username"))
	filter := "(uid=" + username + ")"
	searchReq := ldap.NewSearchRequest(
		"ou=users,dc=example,dc=com",
		ldap.ScopeSingleLevel, ldap.NeverDerefAliases, 10, 30, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01052Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

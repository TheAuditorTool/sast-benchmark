package testcode

import (
	"net/http"
	"os"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01042Conn *ldap.Conn

func BenchmarkTest01042(w http.ResponseWriter, r *http.Request) {
	baseDN := os.Getenv("LDAP_BASE_DN")
	username := ldap.EscapeFilter(r.URL.Query().Get("username"))
	filter := "(uid=" + username + ")"
	searchReq := ldap.NewSearchRequest(
		baseDN,
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"cn", "mail"}, nil,
	)
	result, err := benchmarkTest01042Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

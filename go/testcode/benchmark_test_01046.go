package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01046Conn *ldap.Conn

var benchmarkTest01046AllowedAttrs = map[string]bool{
	"cn": true, "mail": true, "telephoneNumber": true,
}

func BenchmarkTest01046(w http.ResponseWriter, r *http.Request) {
	attr := r.URL.Query().Get("attr")
	if !benchmarkTest01046AllowedAttrs[attr] {
		http.Error(w, "invalid attribute", http.StatusBadRequest)
		return
	}
	value := ldap.EscapeFilter(r.URL.Query().Get("value"))
	filter := "(" + attr + "=" + value + ")"
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		filter, []string{"dn", "cn"}, nil,
	)
	result, err := benchmarkTest01046Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

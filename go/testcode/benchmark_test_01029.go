package testcode

import (
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01029Conn *ldap.Conn

type benchmarkTest01029SearchParams struct {
	Filter string
	BaseDN string
}

func BenchmarkTest01029(w http.ResponseWriter, r *http.Request) {
	params := benchmarkTest01029SearchParams{
		Filter: "(uid=" + r.FormValue("username") + ")",
		BaseDN: "dc=example,dc=com",
	}
	searchReq := ldap.NewSearchRequest(
		params.BaseDN,
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		params.Filter, []string{"cn"}, nil,
	)
	result, err := benchmarkTest01029Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

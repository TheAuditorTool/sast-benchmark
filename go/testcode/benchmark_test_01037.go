package testcode

import (
	"fmt"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01037Conn *ldap.Conn

func BenchmarkTest01037(w http.ResponseWriter, r *http.Request) {
	hostname := r.URL.Query().Get("ldap_host")
	conn, err := ldap.DialURL(fmt.Sprintf("ldap://%s:389", hostname))
	if err != nil {
		http.Error(w, "dial error", http.StatusInternalServerError)
		return
	}
	defer conn.Close()
	searchReq := ldap.NewSearchRequest(
		"dc=example,dc=com",
		ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
		"(objectClass=person)", []string{"cn"}, nil,
	)
	result, _ := conn.Search(searchReq)
	if result == nil {
		RespondJSON(w, http.StatusOK, map[string]int{"count": 0})
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

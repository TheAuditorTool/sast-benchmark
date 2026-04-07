package testcode

import (
	"net/http"
	"sync"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01030Conn *ldap.Conn

func BenchmarkTest01030(w http.ResponseWriter, r *http.Request) {
	username := r.URL.Query().Get("username")
	filter := "(uid=" + username + ")"
	var count int
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		searchReq := ldap.NewSearchRequest(
			"dc=example,dc=com",
			ldap.ScopeWholeSubtree, ldap.NeverDerefAliases, 0, 0, false,
			filter, []string{"dn"}, nil,
		)
		if result, err := benchmarkTest01030Conn.Search(searchReq); err == nil {
			count = len(result.Entries)
		}
	}()
	wg.Wait()
	RespondJSON(w, http.StatusOK, map[string]int{"count": count})
}

package testcode

import (
	"context"
	"net/http"

	"github.com/go-ldap/ldap/v3"
)

var benchmarkTest01031Conn *ldap.Conn

type benchmarkTest01031CtxKey struct{}

func BenchmarkTest01031(w http.ResponseWriter, r *http.Request) {
	ouName := r.URL.Query().Get("ou")
	baseDN := "ou=" + ouName + ",dc=example,dc=com"
	ctx := context.WithValue(r.Context(), benchmarkTest01031CtxKey{}, baseDN)
	dn := ctx.Value(benchmarkTest01031CtxKey{}).(string)
	searchReq := ldap.NewSearchRequest(
		dn,
		ldap.ScopeSingleLevel, ldap.NeverDerefAliases, 0, 0, false,
		"(objectClass=*)", []string{"cn"}, nil,
	)
	result, err := benchmarkTest01031Conn.Search(searchReq)
	if err != nil {
		http.Error(w, "ldap error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": len(result.Entries)})
}

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_escape_all_components
function ldapi045(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $f = ldap_escape($req->param('f'), '', LDAP_ESCAPE_FILTER);
    $dn_part = ldap_escape($req->param('dn'), '', LDAP_ESCAPE_DN);
    $result = ldap_search($conn, "ou=$dn_part,dc=example,dc=com", "(cn=$f)"); // vuln-code-snippet safe-line php_ldapi_escape_all_components
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_escape_all_components

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_allowlist_attr
function ldapi017(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $attr = $req->param('attr');
    $allowed = ['cn', 'mail', 'uid', 'telephoneNumber'];
    if (!in_array($attr, $allowed, true)) {
        return BenchmarkResponse::badRequest('Invalid attribute');
    }
    $result = ldap_search($conn, $base, "(objectClass=person)", [$attr]); // vuln-code-snippet safe-line php_ldapi_allowlist_attr
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_allowlist_attr

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_uuid_lookup
function ldapi043(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dnMap = [
        'sales' => 'ou=sales,dc=example,dc=com',
        'eng' => 'ou=engineering,dc=example,dc=com',
        'hr' => 'ou=hr,dc=example,dc=com',
    ];
    $id = $req->param('id');
    $dn = $dnMap[$id] ?? null; // vuln-code-snippet safe-line php_ldapi_uuid_lookup
    if (!$dn) {
        return BenchmarkResponse::badRequest('unknown department');
    }
    $result = ldap_search($conn, $dn, '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_uuid_lookup

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_attr_and_val_inject
function ldapi028(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $attr = $req->param('attr');
    $val = $req->param('val');
    $filter = "($attr=$val)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_attr_and_val_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_attr_and_val_inject

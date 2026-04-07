<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_base64_decoded_inject
function ldapi033(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $encoded = $req->param('q');
    $decoded = base64_decode($encoded);
    $filter = "(uid=$decoded)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_base64_decoded_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_base64_decoded_inject

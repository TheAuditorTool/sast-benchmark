<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_dn_component_inject
function ldapi020(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dept = $req->param('dept');
    $dn = "ou=$dept,dc=example,dc=com";
    $result = ldap_list($conn, $dn, '(objectClass=*)'); // vuln-code-snippet vuln-line php_ldapi_dn_component_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_dn_component_inject

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_add_injection
function ldapi007(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $cn = $req->post('cn');
    $entry = ['cn' => $cn, 'objectClass' => 'inetOrgPerson', 'sn' => 'User'];
    ldap_add($conn, "cn=" . $cn . ",dc=example,dc=com", $entry); // vuln-code-snippet vuln-line php_ldapi_add_injection
    return BenchmarkResponse::ok('User added');
}
// vuln-code-snippet end php_ldapi_add_injection

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_add_user_dn
function ldapi023(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $cn = $req->param('cn');
    ldap_add($conn, "cn=$cn,ou=users,dc=example,dc=com", ['objectClass' => ['person'], 'cn' => [$cn]]); // vuln-code-snippet vuln-line php_ldapi_add_user_dn
    return BenchmarkResponse::ok('user added');
}
// vuln-code-snippet end php_ldapi_add_user_dn

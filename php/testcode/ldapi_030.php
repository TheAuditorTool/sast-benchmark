<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_passwd_dn_inject
function ldapi030(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $dn = $req->post('dn');
    ldap_exop_passwd($conn, $dn, $req->post('old'), $req->post('new')); // vuln-code-snippet vuln-line php_ldapi_passwd_dn_inject
    return BenchmarkResponse::ok('password changed');
}
// vuln-code-snippet end php_ldapi_passwd_dn_inject

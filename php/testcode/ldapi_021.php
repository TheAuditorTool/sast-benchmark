<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_modify_dn_inject
function ldapi021(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $uid = $req->param('uid');
    ldap_mod_replace($conn, "uid=$uid,ou=users,dc=example,dc=com", ['description' => ['updated']]); // vuln-code-snippet vuln-line php_ldapi_modify_dn_inject
    return BenchmarkResponse::ok('updated');
}
// vuln-code-snippet end php_ldapi_modify_dn_inject

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00501(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $uid = $req->param('uid');
    ldap_mod_replace($conn, "uid=$uid,ou=users,dc=example,dc=com", ['description' => ['updated']]);
    return BenchmarkResponse::ok('updated');
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00135(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $dn = $req->post('dn');
    ldap_exop_passwd($conn, $dn, $req->post('old'), $req->post('new'));
    return BenchmarkResponse::ok('password changed');
}

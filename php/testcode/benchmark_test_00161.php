<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00161(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dn = $req->param('dn');
    $result = ldap_compare($conn, $dn, 'memberOf', 'cn=admins,dc=example,dc=com');
    return BenchmarkResponse::json(['is_admin' => $result]);
}

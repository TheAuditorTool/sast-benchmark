<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00783(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $cn = $req->param('cn');
    ldap_add($conn, "cn=$cn,ou=users,dc=example,dc=com", ['objectClass' => ['person'], 'cn' => [$cn]]);
    return BenchmarkResponse::ok('user added');
}

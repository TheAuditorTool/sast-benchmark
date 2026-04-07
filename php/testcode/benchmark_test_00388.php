<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00388(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_ADMIN_PASS'));
    $cn = ldap_escape($req->param('cn'), '', LDAP_ESCAPE_DN);
    $attrs = ['objectClass' => ['person'], 'cn' => [$cn]];
    ldap_add($conn, "cn=$cn,ou=users,dc=example,dc=com", $attrs);
    return BenchmarkResponse::ok('user added');
}

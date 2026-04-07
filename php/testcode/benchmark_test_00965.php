<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00965(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $cn = $req->post('cn');
    $entry = ['cn' => $cn, 'objectClass' => 'inetOrgPerson', 'sn' => 'User'];
    ldap_add($conn, "cn=" . $cn . ",dc=example,dc=com", $entry);
    return BenchmarkResponse::ok('User added');
}

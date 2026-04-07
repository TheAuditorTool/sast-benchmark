<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00972(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dept = $req->param('dept');
    $dn = "ou=$dept,dc=example,dc=com";
    $result = ldap_list($conn, $dn, '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00507(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $cn = $req->param('cn');
    $safeCn = ldap_escape($cn, '', LDAP_ESCAPE_DN);
    $dn = "cn=" . $safeCn . "," . $base;
    $result = ldap_read($conn, $dn, '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}

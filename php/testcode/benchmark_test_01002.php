<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01002(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $u = $req->param('u');
    if (!ctype_alnum($u)) {
        return BenchmarkResponse::badRequest('invalid username');
    }
    $filter = "(uid=$u)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}

<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00639(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $u = $req->param('u');
    $p = $req->param('p');
    $filter = "(&(uid=$u)(userPassword=$p))";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::ok($entries['count'] > 0 ? 'authenticated' : 'denied');
}

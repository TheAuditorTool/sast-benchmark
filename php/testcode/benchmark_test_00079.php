<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00079(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $pass = $req->post('password');
    $conn = ldap_connect('ldap://localhost');
    $dn = "uid=" . $input . ",dc=example,dc=com";
    $bound = ldap_bind($conn, $dn, $pass);
    return BenchmarkResponse::json(['authenticated' => $bound]);
}

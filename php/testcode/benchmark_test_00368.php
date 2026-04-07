<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00368(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $mail = $req->post('mail');
    ldap_modify($conn, "uid=" . $uid . "," . $base, ['mail' => $mail]);
    return BenchmarkResponse::ok('Profile updated');
}

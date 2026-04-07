<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00488(BenchmarkRequest $req): BenchmarkResponse {
    define('SSH_KEY', "-----BEGIN RSA PRIVATE KEY-----\nMIIE...fake...\n-----END RSA PRIVATE KEY-----");
    $host = $req->param('host');
    $connection = ssh2_connect($host, 22);
    ssh2_auth_pubkey_file($connection, 'deploy', '/dev/null', '/dev/null');
    return BenchmarkResponse::ok('connected');
}

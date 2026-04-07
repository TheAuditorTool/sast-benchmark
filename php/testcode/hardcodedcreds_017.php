<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_ssh_private_key
function hardcodedcreds017(BenchmarkRequest $req): BenchmarkResponse {
    define('SSH_KEY', "-----BEGIN RSA PRIVATE KEY-----\nMIIE...fake...\n-----END RSA PRIVATE KEY-----"); // vuln-code-snippet vuln-line php_hardcoded_ssh_private_key
    $host = $req->param('host');
    $connection = ssh2_connect($host, 22);
    ssh2_auth_pubkey_file($connection, 'deploy', '/dev/null', '/dev/null');
    return BenchmarkResponse::ok('connected');
}
// vuln-code-snippet end php_hardcoded_ssh_private_key

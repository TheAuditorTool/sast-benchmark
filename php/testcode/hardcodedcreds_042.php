<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_cert_fingerprint
function hardcodedcreds042(BenchmarkRequest $req): BenchmarkResponse {
    define('TLS_CERT_FINGERPRINT', 'sha256:AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99'); // vuln-code-snippet safe-line php_hardcoded_cert_fingerprint
    $host = $req->param('host');
    $ctx = stream_context_create(['ssl' => ['peer_fingerprint' => TLS_CERT_FINGERPRINT]]);
    $conn = stream_socket_client('ssl://' . $host . ':443', $errno, $errstr, 5, STREAM_CLIENT_CONNECT, $ctx);
    return BenchmarkResponse::ok($conn ? 'pinned ok' : 'failed: ' . $errstr);
}
// vuln-code-snippet end php_hardcoded_cert_fingerprint

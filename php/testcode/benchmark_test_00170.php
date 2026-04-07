<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00170(BenchmarkRequest $req): BenchmarkResponse {
    $clientCert = $_SERVER['SSL_CLIENT_CERT'] ?? '';
    $verified = $_SERVER['SSL_CLIENT_VERIFY'] ?? 'NONE';
    if ($verified !== 'SUCCESS' || empty($clientCert)) {
        return BenchmarkResponse::error('client certificate required');
    }
    $subject = $_SERVER['SSL_CLIENT_S_DN'] ?? '';
    return BenchmarkResponse::json(['subject' => $subject, 'action' => 'performed']);
}

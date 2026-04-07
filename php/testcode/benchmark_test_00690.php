<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00690(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('ftp_host');
    $conn = ftp_connect($host);
    if ($conn) {
        ftp_login($conn, 'anonymous', 'user@example.com');
        $files = ftp_nlist($conn, '.');
        ftp_close($conn);
        return BenchmarkResponse::json($files);
    }
    return BenchmarkResponse::error('FTP connection failed');
}

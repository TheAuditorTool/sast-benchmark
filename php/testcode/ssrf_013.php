<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_ftp_connect
function ssrf013(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('ftp_host');
    $conn = ftp_connect($host); // vuln-code-snippet vuln-line php_ssrf_ftp_connect
    if ($conn) {
        ftp_login($conn, 'anonymous', 'user@example.com');
        $files = ftp_nlist($conn, '.');
        ftp_close($conn);
        return BenchmarkResponse::json($files);
    }
    return BenchmarkResponse::error('FTP connection failed');
}
// vuln-code-snippet end php_ssrf_ftp_connect

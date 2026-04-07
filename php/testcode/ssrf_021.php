<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_cloud_metadata_reach
function ssrf021(BenchmarkRequest $req): BenchmarkResponse {
    $metaUrl = 'http://169.254.169.254/latest/meta-data/iam/security-credentials/';
    $content = file_get_contents($metaUrl . $req->param('path')); // vuln-code-snippet vuln-line php_ssrf_cloud_metadata_reach
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_ssrf_cloud_metadata_reach

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_stat_info_leak
function pathtraver025(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $info = stat($userPath); // vuln-code-snippet vuln-line php_pt_stat_info_leak
    return BenchmarkResponse::ok(json_encode(['size' => $info['size'] ?? 0]));
}
// vuln-code-snippet end php_pt_stat_info_leak

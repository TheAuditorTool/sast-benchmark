<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_file_put_contents
function pathtraver016(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $data = $req->post('data');
    file_put_contents($userPath, $data); // vuln-code-snippet vuln-line php_pt_file_put_contents
    return BenchmarkResponse::ok('written');
}
// vuln-code-snippet end php_pt_file_put_contents

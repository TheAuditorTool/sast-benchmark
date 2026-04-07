<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00752(BenchmarkRequest $req): BenchmarkResponse {
    $allowedFiles = ['readme' => '/docs/readme.txt', 'license' => '/docs/license.txt', 'changelog' => '/docs/changelog.txt'];
    $fileId = $req->param('file_id');
    if (!isset($allowedFiles[$fileId])) {
        return BenchmarkResponse::badRequest("Unknown file");
    }
    ob_start();
    readfile($allowedFiles[$fileId]);
    $content = ob_get_clean();
    return BenchmarkResponse::ok($content);
}

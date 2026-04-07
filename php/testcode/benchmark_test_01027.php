<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01027(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $noSpecial = str_replace(['-', '_', '.'], '', $filename);
    if (!ctype_alnum($noSpecial)) {
        return BenchmarkResponse::badRequest('invalid filename');
    }
    $content = file_get_contents('/var/app/files/' . $filename);
    return BenchmarkResponse::ok($content);
}

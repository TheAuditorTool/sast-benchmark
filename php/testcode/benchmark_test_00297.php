<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00297(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('action');
    $registered = [
        'export'  => ['ReportService', 'export'],
        'preview' => ['ReportService', 'preview'],
    ];
    if (!isset($registered[$type])) {
        return BenchmarkResponse::badRequest('unknown action');
    }
    $result = call_user_func($registered[$type]);
    return BenchmarkResponse::ok((string) $result);
}

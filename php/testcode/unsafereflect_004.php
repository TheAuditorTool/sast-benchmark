<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_dispatch_validated
function unsafereflect004(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('action');
    $registered = [
        'export'  => ['ReportService', 'export'],
        'preview' => ['ReportService', 'preview'],
    ];
    if (!isset($registered[$type])) { // vuln-code-snippet safe-line php_reflect_dispatch_validated
        return BenchmarkResponse::badRequest('unknown action');
    }
    $result = call_user_func($registered[$type]);
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_reflect_dispatch_validated

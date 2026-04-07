<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00616(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $handlers = [
        'pdf'  => 'PdfHandler',
        'html' => 'HtmlHandler',
    ];
    if (!isset($handlers[$type])) {
        return BenchmarkResponse::badRequest('unsupported type');
    }
    $class = $handlers[$type];
    $result = $class::handle();
    return BenchmarkResponse::ok((string) $result);
}

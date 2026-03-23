<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_enum_map
function unsafereflect006(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $handlers = [
        'pdf'  => 'PdfHandler',
        'html' => 'HtmlHandler',
    ];
    if (!isset($handlers[$type])) { // vuln-code-snippet safe-line php_reflect_enum_map
        return BenchmarkResponse::badRequest('unsupported type');
    }
    $class = $handlers[$type];
    $result = $class::handle();
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_reflect_enum_map

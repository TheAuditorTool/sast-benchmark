<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_int_keyed_handlers
function codeinj040(BenchmarkRequest $req): BenchmarkResponse {
    $handlers = [0 => 'trim', 1 => 'strtolower'];
    $id = intval($req->param('id'));
    $fn = $handlers[$id] ?? 'trim';
    $result = $fn($req->param('value')); // vuln-code-snippet safe-line php_codeinj_int_keyed_handlers
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_int_keyed_handlers

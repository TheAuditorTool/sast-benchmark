<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00347(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('const');
    $allowed = ['PHP_INT_MAX', 'PHP_INT_SIZE', 'PHP_VERSION'];
    if (!in_array($name, $allowed, true)) {
        return BenchmarkResponse::badRequest('Constant not allowed');
    }
    $value = constant($name);
    return BenchmarkResponse::ok((string)$value);
}

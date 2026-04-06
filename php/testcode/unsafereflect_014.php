<?php
require_once __DIR__ . '/shared.php';

interface HandlerInterface014 {
    public function handle(): string;
}

// vuln-code-snippet start php_reflect_instanceof_check
function unsafereflect014(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    if (!class_exists($className)) {
        return BenchmarkResponse::badRequest('Class not found');
    }
    $obj = new $className();
    if (!($obj instanceof HandlerInterface014)) { // vuln-code-snippet safe-line php_reflect_instanceof_check
        return BenchmarkResponse::badRequest('Invalid handler type');
    }
    return BenchmarkResponse::ok($obj->handle());
}
// vuln-code-snippet end php_reflect_instanceof_check

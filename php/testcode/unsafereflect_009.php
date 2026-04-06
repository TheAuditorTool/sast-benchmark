<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_forward_static
class BaseHandler009 {
    public static function execute(): string { return 'executed'; }
}

function unsafereflect009(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $result = forward_static_call([$className, 'execute']); // vuln-code-snippet vuln-line php_reflect_forward_static
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_reflect_forward_static

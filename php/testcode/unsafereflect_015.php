<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_class_exists_allowlist
function unsafereflect015(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $allowed = ['App\\Export', 'App\\Import', 'App\\Report'];
    if (!in_array($className, $allowed, true)) { // vuln-code-snippet safe-line php_reflect_class_exists_allowlist
        return BenchmarkResponse::badRequest('Handler not allowed');
    }
    $handler = new $className();
    return BenchmarkResponse::ok($handler->run());
}
// vuln-code-snippet end php_reflect_class_exists_allowlist

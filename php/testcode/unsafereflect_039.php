<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_container_registered_only
function unsafereflect039(BenchmarkRequest $req): BenchmarkResponse {
    $container = new stdClass();
    $container->bindings = ['user' => UserService::class];
    $cls = $container->bindings[$req->param('service')] ?? null; // vuln-code-snippet safe-line php_reflect_container_registered_only
    if (!$cls) return BenchmarkResponse::badRequest('not registered');
    $obj = new $cls();
    return BenchmarkResponse::ok('resolved');
}
// vuln-code-snippet end php_reflect_container_registered_only

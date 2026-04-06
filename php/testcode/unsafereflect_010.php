<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_string_new
function unsafereflect010(BenchmarkRequest $req): BenchmarkResponse {
    $handlerClass = $req->post('handler');
    $handler = new $handlerClass(); // vuln-code-snippet vuln-line php_reflect_string_new
    return BenchmarkResponse::ok($handler->run());
}
// vuln-code-snippet end php_reflect_string_new

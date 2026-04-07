<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_post_callback_array
function unsafereflect024(BenchmarkRequest $req): BenchmarkResponse {
    $handler = $req->post('handler');
    call_user_func_array($handler, [$req->param('arg')]); // vuln-code-snippet vuln-line php_reflect_post_callback_array
    return BenchmarkResponse::ok('handled');
}
// vuln-code-snippet end php_reflect_post_callback_array

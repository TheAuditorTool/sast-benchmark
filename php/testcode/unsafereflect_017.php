<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_static_map
function unsafereflect017(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $handlers = [
        'greet' => function() { return 'Hello'; },
        'farewell' => function() { return 'Goodbye'; },
        'status' => function() { return 'OK'; },
    ];
    if (!isset($handlers[$action])) {
        return BenchmarkResponse::badRequest('Unknown action');
    }
    $result = $handlers[$action](); // vuln-code-snippet safe-line php_reflect_static_map
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_static_map

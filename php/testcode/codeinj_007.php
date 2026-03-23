<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_create_function
function codeinj_create_function(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->post('body');
    $fn = create_function('$x', $body); // vuln-code-snippet vuln-line php_codeinj_create_function
    $result = $fn(42);
    return BenchmarkResponse::ok("Result: " . $result);
}
// vuln-code-snippet end php_codeinj_create_function

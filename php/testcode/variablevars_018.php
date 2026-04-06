<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_named_args
function formatOutput018(string $name, string $email): string {
    return $name . ' <' . $email . '>';
}

function variablevars018(BenchmarkRequest $req): BenchmarkResponse {
    $result = formatOutput018(
        name: $req->param('name'),
        email: $req->param('email'), // vuln-code-snippet safe-line php_vv_named_args
    );
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_vv_named_args

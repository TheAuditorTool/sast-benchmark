<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_blade_double_brace
function ssti033(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $safe = htmlspecialchars($input, ENT_QUOTES); // vuln-code-snippet safe-line php_ssti_blade_double_brace
    echo "Hello: $safe";
    return BenchmarkResponse::ok($safe);
}
// vuln-code-snippet end php_ssti_blade_double_brace

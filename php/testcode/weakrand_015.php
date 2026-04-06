<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_random_int_range
function weakrand015(BenchmarkRequest $req): BenchmarkResponse {
    $index = random_int(0, 9); // vuln-code-snippet safe-line php_weakrand_random_int_range
    $colors = ['red','blue','green','yellow','purple','orange','pink','white','black','gray'];
    return BenchmarkResponse::json(['selection' => $colors[$index]]);
}
// vuln-code-snippet end php_weakrand_random_int_range

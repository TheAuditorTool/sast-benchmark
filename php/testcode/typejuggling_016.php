<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_filter_validate_int
function typejuggling016(BenchmarkRequest $req): BenchmarkResponse {
    $age = $req->param('age');
    $filtered = filter_var($age, FILTER_VALIDATE_INT, ['options' => ['min_range' => 0, 'max_range' => 150]]); // vuln-code-snippet safe-line php_tj_filter_validate_int
    if ($filtered === false) {
        return BenchmarkResponse::badRequest('Invalid age');
    }
    return BenchmarkResponse::json(['age' => $filtered]);
}
// vuln-code-snippet end php_tj_filter_validate_int

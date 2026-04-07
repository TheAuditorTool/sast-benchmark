<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_typed_int_parameter
function typejuggling041(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    return processId041((int)$id);
}

function processId041(int $userId): BenchmarkResponse {
    if ($userId === 42) { // vuln-code-snippet safe-line php_tj_typed_int_parameter
        return BenchmarkResponse::ok('found');
    }
    return BenchmarkResponse::ok('not found');
}
// vuln-code-snippet end php_tj_typed_int_parameter

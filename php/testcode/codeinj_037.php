<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_eval_json_scalar
function codeinj037(BenchmarkRequest $req): BenchmarkResponse {
    $n = $req->param('n');
    $literal = json_encode($n);
    $val = eval('return ' . $literal . ';'); // vuln-code-snippet safe-line php_codeinj_eval_json_scalar
    return BenchmarkResponse::ok((string) $val);
}
// vuln-code-snippet end php_codeinj_eval_json_scalar

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_literal_method
class Processor043 {
    public function process(string $data): string {
        return htmlspecialchars($data, ENT_QUOTES);
    }
}

function codeinj043(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new Processor043();
    $result = call_user_func([$obj, 'process'], $req->param('data')); // vuln-code-snippet safe-line php_codeinj_literal_method
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_codeinj_literal_method

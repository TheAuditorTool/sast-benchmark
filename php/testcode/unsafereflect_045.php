<?php
require_once __DIR__ . '/shared.php';

class DataProcessor {
    public function process(string $data): string { return strtoupper($data); }
}

// vuln-code-snippet start php_reflect_literal_method_string
function unsafereflect045(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $obj = new DataProcessor();
    $result = call_user_func([$obj, 'process'], $data); // vuln-code-snippet safe-line php_reflect_literal_method_string
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_literal_method_string

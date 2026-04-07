<?php
require_once __DIR__ . '/shared.php';

class SafeValueObject041 {
    public string $name = '';
    public int $value = 0;
}

// vuln-code-snippet start php_deser_single_allowed_class
function deserial041(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $obj = unserialize($raw, ['allowed_classes' => [SafeValueObject041::class]]); // vuln-code-snippet safe-line php_deser_single_allowed_class
    if (!($obj instanceof SafeValueObject041)) {
        return BenchmarkResponse::badRequest('unexpected type');
    }
    return BenchmarkResponse::ok($obj->name . ':' . $obj->value);
}
// vuln-code-snippet end php_deser_single_allowed_class

<?php
require_once __DIR__ . '/shared.php';

class SafeValueObject041 {
    public string $name = '';
    public int $value = 0;
}

function benchmarkTest00238(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $obj = unserialize($raw, ['allowed_classes' => [SafeValueObject041::class]]);
    if (!($obj instanceof SafeValueObject041)) {
        return BenchmarkResponse::badRequest('unexpected type');
    }
    return BenchmarkResponse::ok($obj->name . ':' . $obj->value);
}

<?php
require_once __DIR__ . '/shared.php';

class AuthToken046 {
    public function __construct(private string $value) {}
    public function getValue(): string { return $this->value; }
}

function benchmarkTest00734(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->param('token');
    $token = new AuthToken046($raw);
    if (!($token instanceof AuthToken046)) {
        return BenchmarkResponse::badRequest('invalid token type');
    }
    return BenchmarkResponse::ok($token->getValue());
}

<?php
require_once __DIR__ . '/shared.php';

class AuthToken046 {
    public function __construct(private string $value) {}
    public function getValue(): string { return $this->value; }
}

// vuln-code-snippet start php_tj_instanceof_auth_token
function typejuggling046(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->param('token');
    $token = new AuthToken046($raw);
    if (!($token instanceof AuthToken046)) { // vuln-code-snippet safe-line php_tj_instanceof_auth_token
        return BenchmarkResponse::badRequest('invalid token type');
    }
    return BenchmarkResponse::ok($token->getValue());
}
// vuln-code-snippet end php_tj_instanceof_auth_token

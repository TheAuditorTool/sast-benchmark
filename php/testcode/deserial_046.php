<?php
require_once __DIR__ . '/shared.php';

class Money046 {
    public function __construct(
        public readonly int $amount,
        public readonly string $currency,
    ) {}
}

// vuln-code-snippet start php_deser_readonly_value_object
function deserial046(BenchmarkRequest $req): BenchmarkResponse {
    $json = json_decode($req->bodyStr(), true);
    $money = new Money046((int) $json['amount'], (string) $json['currency']); // vuln-code-snippet safe-line php_deser_readonly_value_object
    return BenchmarkResponse::json(['amount' => $money->amount, 'currency' => $money->currency]);
}
// vuln-code-snippet end php_deser_readonly_value_object

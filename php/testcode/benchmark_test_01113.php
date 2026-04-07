<?php
require_once __DIR__ . '/shared.php';

class Money046 {
    public function __construct(
        public readonly int $amount,
        public readonly string $currency,
    ) {}
}

function benchmarkTest01113(BenchmarkRequest $req): BenchmarkResponse {
    $json = json_decode($req->bodyStr(), true);
    $money = new Money046((int) $json['amount'], (string) $json['currency']);
    return BenchmarkResponse::json(['amount' => $money->amount, 'currency' => $money->currency]);
}

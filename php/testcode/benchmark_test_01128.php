<?php
require_once __DIR__ . '/shared.php';

class OrderDTO040 {
    public function __construct(
        public readonly int $id,
        public readonly int $qty,
    ) {}
}

function benchmarkTest01128(BenchmarkRequest $req): BenchmarkResponse {
    $json = json_decode($req->bodyStr(), true);
    $dto = new OrderDTO040((int) $json['id'], (int) $json['qty']);
    return BenchmarkResponse::json(['id' => $dto->id, 'qty' => $dto->qty]);
}

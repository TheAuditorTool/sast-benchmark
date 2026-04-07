<?php
require_once __DIR__ . '/shared.php';

class OrderDTO040 {
    public function __construct(
        public readonly int $id,
        public readonly int $qty,
    ) {}
}

// vuln-code-snippet start php_deser_dto_constructor
function deserial040(BenchmarkRequest $req): BenchmarkResponse {
    $json = json_decode($req->bodyStr(), true);
    $dto = new OrderDTO040((int) $json['id'], (int) $json['qty']); // vuln-code-snippet safe-line php_deser_dto_constructor
    return BenchmarkResponse::json(['id' => $dto->id, 'qty' => $dto->qty]);
}
// vuln-code-snippet end php_deser_dto_constructor

<?php
require_once __DIR__ . '/shared.php';

class SafeDTO {
    public string $name;
    public int $age;
}

function benchmarkTest01081(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data, ['allowed_classes' => ['SafeDTO']]);
    return BenchmarkResponse::json(['name' => $obj->name ?? 'unknown']);
}

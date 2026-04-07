<?php
require_once __DIR__ . '/shared.php';

class SafeDTO017 {
    public string $name;
    public string $email;
}

function benchmarkTest00985(BenchmarkRequest $req): BenchmarkResponse {
    $raw = base64_decode($req->post('payload'));
    $obj = igbinary_unserialize($raw);
    if (!($obj instanceof SafeDTO017)) {
        return BenchmarkResponse::badRequest('Invalid payload type');
    }
    return BenchmarkResponse::json(['name' => $obj->name, 'email' => $obj->email]);
}

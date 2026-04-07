<?php
require_once __DIR__ . '/shared.php';

class SafeContainer016 implements ArrayAccess {
    private array $data = [];
    public function offsetExists(mixed $offset): bool { return isset($this->data[$offset]); }
    public function offsetGet(mixed $offset): mixed { return $this->data[$offset] ?? null; }
    public function offsetSet(mixed $offset, mixed $value): void { $this->data[$offset] = $value; }
    public function offsetUnset(mixed $offset): void { unset($this->data[$offset]); }
}

function benchmarkTest00728(BenchmarkRequest $req): BenchmarkResponse {
    $container = new SafeContainer016();
    $key = $req->param('key');
    $container[$key] = $req->param('value');
    return BenchmarkResponse::json(['stored' => $container[$key]]);
}

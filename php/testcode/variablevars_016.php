<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_array_access
class SafeContainer016 implements ArrayAccess {
    private array $data = [];
    public function offsetExists(mixed $offset): bool { return isset($this->data[$offset]); }
    public function offsetGet(mixed $offset): mixed { return $this->data[$offset] ?? null; }
    public function offsetSet(mixed $offset, mixed $value): void { $this->data[$offset] = $value; }
    public function offsetUnset(mixed $offset): void { unset($this->data[$offset]); }
}

function variablevars016(BenchmarkRequest $req): BenchmarkResponse {
    $container = new SafeContainer016();
    $key = $req->param('key');
    $container[$key] = $req->param('value'); // vuln-code-snippet safe-line php_vv_array_access
    return BenchmarkResponse::json(['stored' => $container[$key]]);
}
// vuln-code-snippet end php_vv_array_access

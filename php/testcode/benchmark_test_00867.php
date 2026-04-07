<?php
require_once __DIR__ . '/shared.php';

class ChainLink029 {
    public mixed $next = null;
    public string $method = '';
    public function __wakeup(): void {
        if ($this->next !== null) {
            $m = $this->method;
            $this->next->$m();
        }
    }
    public function __call(string $name, array $args): mixed {
        return call_user_func_array([$this->next, $name], $args);
    }
}

function benchmarkTest00867(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data);
    return BenchmarkResponse::ok('processed');
}

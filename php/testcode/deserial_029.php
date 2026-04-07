<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_pop_chain
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

function deserial029(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('data');
    $obj = unserialize($data); // vuln-code-snippet vuln-line php_deser_pop_chain
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_pop_chain

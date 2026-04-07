<?php
require_once __DIR__ . '/shared.php';

class SqlGadget023 {
    public mixed $pdo = null;
    public string $sql = '';
    public function __wakeup(): void {
        $this->pdo->exec($this->sql);
    }
}

function benchmarkTest00180(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('data');
    $obj = unserialize($input);
    return BenchmarkResponse::ok('processed');
}

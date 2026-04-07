<?php
require_once __DIR__ . '/shared.php';

class ReadOnlyModel036 {
    public function __set(string $name, mixed $value): void {
        throw new \RuntimeException("Model is read-only; cannot set '$name'");
    }
}

function benchmarkTest01133(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int) $req->param('id');
    $row = $db->query("SELECT name, email FROM users WHERE id = $id")->fetch();
    return BenchmarkResponse::json(['name' => $row['name'], 'email' => $row['email']]);
}

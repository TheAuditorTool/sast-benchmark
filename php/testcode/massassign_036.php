<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_readonly_model
class ReadOnlyModel036 {
    public function __set(string $name, mixed $value): void {
        throw new \RuntimeException("Model is read-only; cannot set '$name'");
    }
}

function massassign036(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int) $req->param('id');
    $row = $db->query("SELECT name, email FROM users WHERE id = $id")->fetch(); // vuln-code-snippet safe-line php_massassign_readonly_model
    return BenchmarkResponse::json(['name' => $row['name'], 'email' => $row['email']]);
}
// vuln-code-snippet end php_massassign_readonly_model

<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_wakeup_sql_gadget
class SqlGadget023 {
    public mixed $pdo = null;
    public string $sql = '';
    public function __wakeup(): void {
        $this->pdo->exec($this->sql);
    }
}

function deserial023(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->post('data');
    $obj = unserialize($input); // vuln-code-snippet vuln-line php_deser_wakeup_sql_gadget
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_wakeup_sql_gadget

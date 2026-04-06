<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_is_numeric_hex
function typejuggling013(BenchmarkRequest $req): BenchmarkResponse {
    $amount = $req->param('amount');
    if (!is_numeric($amount)) { // vuln-code-snippet vuln-line php_tj_is_numeric_hex
        return BenchmarkResponse::badRequest('Amount must be numeric');
    }
    $pdo = getDbConnection();
    $pdo->prepare("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        ->execute([$amount, $req->param('account_id')]);
    return BenchmarkResponse::ok('Transaction processed');
}
// vuln-code-snippet end php_tj_is_numeric_hex

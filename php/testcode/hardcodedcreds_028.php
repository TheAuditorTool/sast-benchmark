<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_install_admin_pass
function hardcodedcreds028(BenchmarkRequest $req): BenchmarkResponse {
    $adminPass = 'admin123'; // vuln-code-snippet vuln-line php_hardcoded_install_admin_pass
    $username = $req->param('username');
    $pdo = getDbConnection();
    $hash = password_hash($adminPass, PASSWORD_BCRYPT);
    $stmt = $pdo->prepare('INSERT INTO users (username, password) VALUES (?, ?)');
    $stmt->execute([$username, $hash]);
    return BenchmarkResponse::ok('admin created');
}
// vuln-code-snippet end php_hardcoded_install_admin_pass

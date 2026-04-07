<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_admin_gate_role
function massassign041(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name'] ?? '';
    if ($_SESSION['is_admin']) { // vuln-code-snippet safe-line php_massassign_admin_gate_role
        $model->role = $_POST['role'];
    }
    return BenchmarkResponse::ok('gated');
}
// vuln-code-snippet end php_massassign_admin_gate_role

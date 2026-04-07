<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_event_sourcing_immutable
function massassign046(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = new stdClass();
    $cmd->name = $_POST['name'] ?? ''; // vuln-code-snippet safe-line php_massassign_event_sourcing_immutable
    $cmd->email = $_POST['email'] ?? '';
    return BenchmarkResponse::ok('command dispatched');
}
// vuln-code-snippet end php_massassign_event_sourcing_immutable

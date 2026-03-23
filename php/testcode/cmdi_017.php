<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_multihop
function runCommand(string $cmd): string {
    $output = [];
    exec($cmd, $output); // vuln-code-snippet vuln-line php_cmdi_multihop
    return implode("\n", $output);
}

function cmdi017(BenchmarkRequest $req): BenchmarkResponse {
    $userCmd = "ls " . $req->param('dir');
    $result = runCommand($userCmd);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_cmdi_multihop

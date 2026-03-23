<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_extension_enforced
function fileinclusion010(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('module');
    $safe = basename($file);
    $ext = pathinfo($safe, PATHINFO_EXTENSION);
    if ($ext !== 'php') {
        return BenchmarkResponse::badRequest("only .php modules allowed");
    }
    $allowedModules = ['utils.php', 'helpers.php', 'config.php'];
    if (!in_array($safe, $allowedModules, true)) {
        return BenchmarkResponse::badRequest("module not in allowlist");
    }
    include("modules/" . $safe); // vuln-code-snippet safe-line php_fi_extension_enforced
    return BenchmarkResponse::ok("module loaded");
}
// vuln-code-snippet end php_fi_extension_enforced

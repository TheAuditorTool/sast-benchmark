<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_mkdir
function pathtraver_mkdir(BenchmarkRequest $req): BenchmarkResponse {
    $projectName = $req->param('project');
    mkdir("/projects/" . $projectName, 0755, true); // vuln-code-snippet vuln-line php_pt_mkdir
    return BenchmarkResponse::ok("Project directory created");
}
// vuln-code-snippet end php_pt_mkdir

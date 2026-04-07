<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00554(BenchmarkRequest $req): BenchmarkResponse {
    $projectName = $req->param('project');
    mkdir("/projects/" . $projectName, 0755, true);
    return BenchmarkResponse::ok("Project directory created");
}

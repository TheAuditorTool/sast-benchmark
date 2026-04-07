<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00789(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('project');
    $safeName = preg_replace('/[^a-zA-Z0-9_-]/', '', $name);
    mkdir("/projects/" . $safeName, 0755, true);
    return BenchmarkResponse::ok("Project directory created");
}

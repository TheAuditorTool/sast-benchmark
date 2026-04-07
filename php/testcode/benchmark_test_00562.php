<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00562(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $dh = opendir($userDir);
    $files = [];
    while (($file = readdir($dh)) !== false) {
        $files[] = $file;
    }
    closedir($dh);
    return BenchmarkResponse::ok(implode("\n", $files));
}

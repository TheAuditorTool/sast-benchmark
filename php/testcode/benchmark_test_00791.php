<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00791(BenchmarkRequest $req): BenchmarkResponse {
    $archive = $req->param('archive');
    $entry   = $req->param('entry');
    include 'zip://' . $archive . '#' . $entry;
    return BenchmarkResponse::ok('Done');
}

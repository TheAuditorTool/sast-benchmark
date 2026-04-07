<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00981(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $handlers = [
        'greet' => function() { return 'Hello'; },
        'farewell' => function() { return 'Goodbye'; },
        'status' => function() { return 'OK'; },
    ];
    if (!isset($handlers[$action])) {
        return BenchmarkResponse::badRequest('Unknown action');
    }
    $result = $handlers[$action]();
    return BenchmarkResponse::ok($result);
}

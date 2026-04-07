<?php
require_once __DIR__ . '/shared.php';

interface HandlerInterface {
    public function handle(string $data): string;
}

class SafePdfHandler implements HandlerInterface {
    public function handle(string $data): string { return 'pdf:' . $data; }
}

class SafeCsvHandler implements HandlerInterface {
    public function handle(string $data): string { return 'csv:' . $data; }
}

function benchmarkTest00381(BenchmarkRequest $req): BenchmarkResponse {
    $handlers = ['pdf' => new SafePdfHandler(), 'csv' => new SafeCsvHandler()];
    $h = $handlers[$req->param('type')] ?? null;
    if (!($h instanceof HandlerInterface)) return BenchmarkResponse::badRequest('invalid');
    $result = $h->handle($req->param('data'));
    return BenchmarkResponse::ok($result);
}

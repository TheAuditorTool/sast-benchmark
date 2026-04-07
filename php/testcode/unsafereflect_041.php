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

// vuln-code-snippet start php_reflect_typed_interface_strategy
function unsafereflect041(BenchmarkRequest $req): BenchmarkResponse {
    $handlers = ['pdf' => new SafePdfHandler(), 'csv' => new SafeCsvHandler()];
    $h = $handlers[$req->param('type')] ?? null;
    if (!($h instanceof HandlerInterface)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_reflect_typed_interface_strategy
    $result = $h->handle($req->param('data'));
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_typed_interface_strategy

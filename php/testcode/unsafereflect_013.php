<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_match_dispatch
class ExportAction013 { public function run(): string { return 'exported'; } }
class ImportAction013 { public function run(): string { return 'imported'; } }
class ReportAction013 { public function run(): string { return 'reported'; } }

function unsafereflect013(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $handler = match($action) { // vuln-code-snippet safe-line php_reflect_match_dispatch
        'export' => new ExportAction013(),
        'import' => new ImportAction013(),
        'report' => new ReportAction013(),
        default => null,
    };
    if ($handler === null) {
        return BenchmarkResponse::badRequest('Unknown action');
    }
    return BenchmarkResponse::ok($handler->run());
}
// vuln-code-snippet end php_reflect_match_dispatch

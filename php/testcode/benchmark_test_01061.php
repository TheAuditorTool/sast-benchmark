<?php
require_once __DIR__ . '/shared.php';

class TypedRecord045 {
    public int $id;
    public string $name;
    public array $tags;
}

function benchmarkTest01061(BenchmarkRequest $req): BenchmarkResponse {
    $raw = json_decode($req->bodyStr(), true);
    if (!is_array($raw)) {
        return BenchmarkResponse::badRequest('expected object');
    }
    $rec = new TypedRecord045();
    $rec->id   = (int) ($raw['id'] ?? 0);
    $rec->name = (string) ($raw['name'] ?? '');
    $rec->tags = array_map('strval', $raw['tags'] ?? []);
    return BenchmarkResponse::json(['id' => $rec->id, 'name' => $rec->name]);
}

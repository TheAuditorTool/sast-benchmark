<?php
require_once __DIR__ . '/shared.php';

class UserDTO012 {
    public function __construct(
        public readonly string $name,
        public readonly string $email,
    ) {}
}

function benchmarkTest00120(BenchmarkRequest $req): BenchmarkResponse {
    $dto = new UserDTO012(
        name: $req->post('name'),
        email: $req->post('email'),
    );
    return BenchmarkResponse::json(['name' => $dto->name, 'email' => $dto->email]);
}

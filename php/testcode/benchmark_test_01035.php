<?php
require_once __DIR__ . '/shared.php';

class UserModel007 {
    public string $name = '';
    public string $email = '';
    public string $role = 'user';

    public function __set(string $prop, mixed $value): void {
        $this->$prop = $value;
    }
}

function benchmarkTest01035(BenchmarkRequest $req): BenchmarkResponse {
    $user = new UserModel007();
    foreach ($req->postData as $key => $value) {
        $user->$key = $value;
    }
    return BenchmarkResponse::json(['name' => $user->name, 'role' => $user->role]);
}

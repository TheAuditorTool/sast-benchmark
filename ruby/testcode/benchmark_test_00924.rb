require_relative 'shared'

def api_scoped_action(req)
  token_scope = req.header('X-Token-Scope')
  return BenchmarkResponse.error('forbidden', 403) unless token_scope.include?('admin')
  BenchmarkResponse.ok('admin action performed')
end

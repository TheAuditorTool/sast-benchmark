require_relative 'shared'
require 'bcrypt'

def login_empty_bypass(req)
  username = req.post('username')
  password = req.post('password')
  stored_hash = '$2a$12$examplehashedpasswordvalue'
  return BenchmarkResponse.error('Unauthorized', 401) if password.empty?
  return BenchmarkResponse.error('Unauthorized', 401) unless password == stored_hash
  BenchmarkResponse.ok("Welcome #{username}")
end

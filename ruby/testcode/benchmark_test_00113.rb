require_relative 'shared'
require 'base64'

VALID_USER = 'admin'.freeze
VALID_PASS = 'supersecret'.freeze

def authenticate_basic(req)
  auth_header = req.header('Authorization')
  return BenchmarkResponse.error('Unauthorized', 401) unless auth_header.start_with?('Basic ')
  credentials = Base64.decode64(auth_header.sub('Basic ', ''))
  username, password = credentials.split(':', 2)
  return BenchmarkResponse.error('Unauthorized', 401) unless username == VALID_USER && password == VALID_PASS
  BenchmarkResponse.ok("Welcome #{username}")
end

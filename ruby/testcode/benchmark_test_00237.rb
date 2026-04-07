require_relative 'shared'

TRUSTED_IP = '10.0.0.1'.freeze

def authenticate_by_ip(req)
  remote_addr = req.header('REMOTE_ADDR')
  return BenchmarkResponse.error('Unauthorized', 401) unless remote_addr == TRUSTED_IP
  BenchmarkResponse.ok('Access granted')
end

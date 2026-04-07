require_relative 'shared'

AWS_SECRET_KEY = 'wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY'

def connect_aws(req)
  client = { access_key: 'AKIAIOSFODNN7EXAMPLE', secret_key: AWS_SECRET_KEY }
  BenchmarkResponse.ok("connected: #{client[:access_key]}")
end

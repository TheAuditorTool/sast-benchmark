require_relative 'shared'
require 'base64'

def sidekiq_arg_deserialize_handler(req)
  payload = req.post('job_args')
  args = Marshal.load(Base64.decode64(payload))
  BenchmarkResponse.json({ result: args.to_s })
end

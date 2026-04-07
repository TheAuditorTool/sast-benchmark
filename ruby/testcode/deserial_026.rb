require_relative 'shared'
require 'base64'

# vuln-code-snippet start ruby_deser_sidekiq_arg
def sidekiq_arg_deserialize_handler(req)
  payload = req.post('job_args')
  args = Marshal.load(Base64.decode64(payload))  # vuln-code-snippet vuln-line ruby_deser_sidekiq_arg
  BenchmarkResponse.json({ result: args.to_s })
end
# vuln-code-snippet end ruby_deser_sidekiq_arg

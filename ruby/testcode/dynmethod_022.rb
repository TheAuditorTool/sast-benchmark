require_relative 'shared'

begin
  require 'fileutils'
rescue LoadError
end

# vuln-code-snippet start ruby_dynmethod_public_send_sys
def file_action(req)
  action = req.param('action')
  path = req.param('path')
  FileUtils.public_send(action, path) # vuln-code-snippet vuln-line ruby_dynmethod_public_send_sys
  BenchmarkResponse.json({ result: "ok" })
end
# vuln-code-snippet end ruby_dynmethod_public_send_sys

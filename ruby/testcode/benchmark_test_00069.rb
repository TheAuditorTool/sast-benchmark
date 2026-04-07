require_relative 'shared'
require 'open3'

def resolve_hostname(req)
  hostname = req.param('hostname')
  stdout, _stderr, _status = Open3.capture2("nslookup " + hostname)
  BenchmarkResponse.ok(stdout)
end

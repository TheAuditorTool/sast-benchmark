require_relative 'shared'

begin
  require 'fileutils'
rescue LoadError
end

def file_action(req)
  action = req.param('action')
  path = req.param('path')
  FileUtils.public_send(action, path)
  BenchmarkResponse.json({ result: "ok" })
end

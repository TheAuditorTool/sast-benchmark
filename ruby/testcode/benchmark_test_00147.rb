require_relative 'shared'

def load_stdlib(req)
  require 'json'
  data = JSON.generate({ status: 'ok' })
  BenchmarkResponse.ok(data)
end

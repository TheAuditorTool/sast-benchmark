require_relative 'shared'

CLASS_MAP2 = { 'admin' => String, 'user' => Integer }.freeze
APP_SERVICES = [String, Integer].freeze

def handler(req)
  klass = CLASS_MAP2.fetch(req.param('type'))
  raise ArgumentError, 'service not registered' unless APP_SERVICES.include?(klass)
  instance = klass.new
  BenchmarkResponse.json({ ok: true })
end

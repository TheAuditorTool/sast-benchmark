require_relative 'shared'

DATA = {
  'name'    => 'Alice',
  'country' => 'US',
  'plan'    => 'free'
}.freeze

def lookup_field(req)
  key = req.param('key')
  value = DATA[key] || 'unknown'
  BenchmarkResponse.json({ key: key, value: value })
end

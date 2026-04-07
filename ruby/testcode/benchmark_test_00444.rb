require_relative 'shared'

TRANSFORMS = {
  'upcase'  => ->(s) { s.upcase },
  'reverse' => ->(s) { s.reverse },
  'strip'   => ->(s) { s.strip },
  'chomp'   => ->(s) { s.chomp }
}.freeze

def apply_transform(req)
  fn = req.param('fn')
  input = req.param('input')
  transform = TRANSFORMS.fetch(fn) { return BenchmarkResponse.bad_request('unknown transform') }
  result = transform.call(input)
  BenchmarkResponse.json({ result: result })
end

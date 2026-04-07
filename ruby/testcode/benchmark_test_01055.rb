require_relative 'shared'

CONTAINER_046 = {
  'logger' => -> { 'log' },
  'mailer' => -> { 'mail' }
}.freeze

def dry_container_dispatch(req)
  service = req.param('service')
  result  = CONTAINER_046.fetch(service).call
  BenchmarkResponse.json({ result: result })
end

require_relative 'shared'

def header_redirect(req)
  next_url = req.param('next')
  BenchmarkResponse.new(302, '', { 'Location' => next_url })
end
